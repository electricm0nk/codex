"""Re-derives artifacts/epic-5-forward-plan/forward-plan.json (AT-34-E5-001).

Run from anywhere inside the repo:
    python3 docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/build_forward_plan.py

Reads docs/work-inventory.json (live, at HEAD -- decisions.md §12 L2: never carry a number
forward, re-derive it) and the measured rates cited inline in RATES below, each with its
own source artifact and sample size.
"""
import json, subprocess, sys, os

REPO = subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True, text=True, check=True).stdout.strip()
sys.path.insert(0, os.path.join(REPO, "scripts"))
import completion_atlas as ca

HEAD = subprocess.run(["git","rev-parse","HEAD"], cwd=REPO, capture_output=True, text=True, check=True).stdout.strip()

inv = json.load(open(f"{REPO}/docs/work-inventory.json"))
units = inv["units"]

VEHICLE_BOOKS = {"core_rulebook", "ultimate_campaign"}
ALL_BOOKS = sorted({u["book"] for u in units})
PLAN_BOOKS = [b for b in ALL_BOOKS if b not in VEHICLE_BOOKS]
assert len(ALL_BOOKS) == 37, len(ALL_BOOKS)
assert len(PLAN_BOOKS) == 35, len(PLAN_BOOKS)

MECHANISM = {
    "A": "build (or extend) the missing engine table for this kind",
    "B": "place the record in the existing table",
    "C": "wire the display/explanation path to the already-held, already-computed value",
    "D": "per named sub-cause (not yet enumerated outside the two vehicle books)",
    "M": "run the compute path against the ingested magnitude",
    "V": "SD-33 oracle harness verification (replace proxy-verification with oracle agreement)",
    "U": "instrument correction (fixes a class of units at once, not per-record)",
    "X": "revisit the stated deferral condition",
    "Z": "ordinary ingestion-to-grounded work (no dedicated SD-34 cycle measured this bucket)",
}

RATES = {
    "A": {
        "metric": "seconds_per_table_build",
        "value": 136.857,
        "sample_size": 7,
        "sample_description": "7 of Epic 2's 8 tables (ability/template/trait/deity/domain/skill/language); companion excluded (pre-existing SD-29 table, 0 build time this bundle)",
        "source": "artifacts/epic-2-tables/table-build-rate.json .tables[].est_wall_time_seconds",
        "thin_sample": False,
        "caveat": "Epic 2's own figures are marked ESTIMATE: pro-rated from measured marginal lines against one shared-cycle wall time, not independently timed per table (table-build-rate.json measurement_note). This is a TABLE-BUILD cost, not a per-unit cost -- once built, the records inside still need their own bucket-clearing mechanism (usually B) unless the same generic loader both creates the table and ingests its records, as it did for Epic 2's 7 new kinds.",
    },
    "B": {
        "metric": "units_per_hour_reaching_DONE",
        "range": [1.667, 5.8],
        "sample_size": "2 measurements: core_rulebook (29 cycles, 503 net units, 235 reached DONE) and ultimate_campaign (1 cycle, 5 units, 3 reached DONE)",
        "source": "artifacts/epic-3-core-rulebook/step-cost-ledger.json .buckets_cleared_so_far.B; artifacts/epic-4-ultimate-campaign/step-cost-ledger.json .buckets_cleared_so_far.B",
        "thin_sample": True,
        "caveat": "artifacts/epic-4-ultimate-campaign/step-cost-ledger.json's own comparison concludes the 3.5x divergence between the two rates is a SAMPLE-SIZE artifact (Epic 4's n=1 cycle/5 units), not a book-shape effect, and that Core Rulebook's own per-mechanism spread is 22.2-617.4 min/cycle (28x) -- a single blended units/hour figure materially misrepresents any individual book's true cost. Use the range, never a point estimate.",
    },
    "C": {
        "metric": "units_per_hour_net_reclassified",
        "value": 41.3,
        "units_per_hour_reaching_DONE": 0.0,
        "sample_size": 1,
        "sample_description": "1 cycle, core_rulebook only (42 units left bucket C in 61 minutes, ALL moved to bucket V, none reached DONE directly)",
        "source": "artifacts/epic-3-core-rulebook/step-cost-ledger.json .buckets_cleared_so_far.C",
        "thin_sample": True,
        "caveat": "This is the only bucket-C cycle SD-34 has run. It measures wiring-to-V, not wiring-to-DONE (this book's Favored Enemy/Terrain mechanism grounds the compute path but the result still needs the oracle harness). A book whose bucket-C mechanism wires straight to a player-facing surface (not through V) would have a different, unmeasured rate. n=1 mechanism -- do not extrapolate to a book-wide bucket-C rate.",
    },
    "U": {
        "metric": "units_per_hour_reaching_DONE",
        "value": 40.28,
        "sample_size": 2,
        "sample_description": "2 cycles, core_rulebook only (48 units, 71.5 wall minutes)",
        "source": "artifacts/epic-3-core-rulebook/step-cost-ledger.json .buckets_cleared_so_far.U",
        "thin_sample": True,
        "caveat": "Both measured cycles were instrument corrections that each fixed a WHOLE CLASS of units at once (a rendering defect and a ruling), not per-record labor -- the 'units/hour' framing is a byproduct of how many units that particular class happened to contain, not a marginal per-unit cost. A book's bucket-U units may or may not share a fixable class with core_rulebook's; no per-book multiplier exists.",
    },
    "D": {"metric": None, "sample_size": 0, "caveat": "UNMEASURED -- decisions.md §2 defines D as cleared 'per named sub-cause', and zero dedicated D-clearing cycles have run in either vehicle book (step-cost-ledger.json buckets_not_yet_cleared.D, both books). No rate exists to project from."},
    "M": {"metric": None, "sample_size": 0, "caveat": "UNMEASURED -- zero dedicated M-clearing (compute-and-apply) cycles have run in either vehicle book (step-cost-ledger.json buckets_not_yet_cleared.M, both books). AT-34-E4-002's own receipt flags this bucket as requiring a sample measurement before any population run; that measurement has not yet happened."},
    "V": {"metric": None, "sample_size": 0, "caveat": "UNMEASURED -- zero dedicated V-clearing (oracle-harness) cycles have run in either vehicle book (step-cost-ledger.json buckets_not_yet_cleared.V, both books). The 42 units that reached V this bundle (core_rulebook bucket C cycle) arrived AS A SIDE EFFECT of a C-clearing cycle, not from a V-clearing cycle -- no bucket-V-clearing rate exists."},
    "X": {"metric": None, "sample_size": 0, "caveat": "UNMEASURED -- ultimate_campaign's 2 X units were resolved BY PROOF (AT-34-E4-001), which moves 0 units in the ledger's bucket-count sense and produces no rate (step-cost-ledger.json buckets_not_yet_cleared.X). No book has a rate-generating X-clearing cycle."},
    "Z": {"metric": None, "sample_size": 0, "caveat": "UNMEASURED -- Z ('not started') has no dedicated SD-34 clearing cycle in either vehicle book (both vehicle books show Z=0). All 19 SD-34 Z units are in beginner_box; nothing in this bundle establishes a per-unit rate for taking a not-started unit to grounded."},
}

def price_bucket(bucket, count):
    r = RATES[bucket]
    if r.get("metric") is None:
        return {"unit_count": count, "mechanism": MECHANISM[bucket], "rate": None,
                "sample_size": 0, "projected_cost_hours": None,
                "note": r["caveat"]}
    if bucket == "A":
        tables_needed = 1  # one table serves the whole kind's population in this book
        secs = r["value"] * tables_needed
        return {"unit_count": count, "mechanism": MECHANISM[bucket],
                "rate": {"metric": r["metric"], "value": r["value"], "source": r["source"]},
                "sample_size": r["sample_size"],
                "projected_cost_hours": round(secs/3600, 4),
                "note": r["caveat"] + " Cost priced as ONE table build (Epic 2's per-table figure), not per-unit -- a table's cost does not scale with how many records fall into it."}
    if bucket == "B":
        lo, hi = r["range"]
        hrs_lo, hrs_hi = round(count/hi, 3), round(count/lo, 3)
        return {"unit_count": count, "mechanism": MECHANISM[bucket],
                "rate": {"metric": r["metric"], "range_units_per_hour": r["range"], "source": r["source"]},
                "sample_size": r["sample_size"],
                "projected_cost_hours_range": [hrs_lo, hrs_hi],
                "note": r["caveat"]}
    if bucket == "C":
        hrs = round(count / r["value"], 3)
        return {"unit_count": count, "mechanism": MECHANISM[bucket],
                "rate": {"metric": r["metric"], "value": r["value"], "units_per_hour_reaching_DONE": r["units_per_hour_reaching_DONE"], "source": r["source"]},
                "sample_size": r["sample_size"],
                "projected_cost_hours_to_reach_V_not_DONE": hrs,
                "projected_cost_hours_to_reach_DONE": None,
                "note": r["caveat"]}
    if bucket == "U":
        hrs = round(count / r["value"], 3)
        return {"unit_count": count, "mechanism": MECHANISM[bucket],
                "rate": {"metric": r["metric"], "value": r["value"], "source": r["source"]},
                "sample_size": r["sample_size"],
                "projected_cost_hours": hrs,
                "note": r["caveat"]}
    raise AssertionError(bucket)

books_out = []
totals_by_bucket = {}
priced_hours_lo = 0.0
priced_hours_hi = 0.0
priced_to_v_not_done_units = 0
priced_to_v_not_done_hours = 0.0
unpriced_units = 0
for book in PLAN_BOOKS:
    pres = ca.partition(units, book=book)
    counts, unclassified, overlap, examined = pres["counts"], pres["unclassified_ids"], pres["overlap_ids"], pres["examined"]
    assert not unclassified and not overlap, (book, unclassified, overlap)
    total = examined
    done = counts.get("DONE", 0)
    remaining = total - done
    buckets_out = {}
    for b in ca.BUCKET_ORDER:
        if b == "DONE":
            continue
        n = counts.get(b, 0)
        if n == 0:
            continue
        priced = price_bucket(b, n)
        buckets_out[b] = priced
        totals_by_bucket[b] = totals_by_bucket.get(b, 0) + n
        if "projected_cost_hours" in priced and priced["projected_cost_hours"] is not None:
            priced_hours_lo += priced["projected_cost_hours"]
            priced_hours_hi += priced["projected_cost_hours"]
        elif "projected_cost_hours_range" in priced:
            priced_hours_lo += priced["projected_cost_hours_range"][0]
            priced_hours_hi += priced["projected_cost_hours_range"][1]
        elif "projected_cost_hours_to_reach_V_not_DONE" in priced:
            # Bucket C: priced to reach V (a real, measured rate), but NOT to reach DONE --
            # counted separately so it is neither silently folded into the DONE-hours range
            # nor miscounted as having no rate at all (it has one, for a different endpoint).
            priced_to_v_not_done_units += n
            priced_to_v_not_done_hours += priced["projected_cost_hours_to_reach_V_not_DONE"]
        else:
            unpriced_units += n
    books_out.append({
        "book": book, "total_units": total, "done": done, "remaining_non_done": remaining,
        "buckets": buckets_out,
    })

remaining_total = sum(bk["remaining_non_done"] for bk in books_out)
assert remaining_total == sum(totals_by_bucket.values())

out = {
    "criterion": "AT-34-E5-001",
    "generated_at_head": HEAD,
    "re_derive_command": "python3 docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/build_forward_plan.py",
    "population": {
        "description": "35 non-vehicle books (37 total inventory books minus core_rulebook and ultimate_campaign)",
        "books_count": len(PLAN_BOOKS),
        "total_units_35_books": sum(bk["total_units"] for bk in books_out),
        "done_units_35_books": sum(bk["done"] for bk in books_out),
        "remaining_non_done_units_35_books": remaining_total,
        "remaining_by_bucket": totals_by_bucket,
        "re_derive_command": "python3 scripts/completion_atlas.py --by-book",
    },
    "measured_rates": RATES,
    "books": books_out,
    "summary": {
        "priced_to_done_units": remaining_total - unpriced_units - priced_to_v_not_done_units,
        "priced_to_v_not_done_units": priced_to_v_not_done_units,
        "priced_to_v_not_done_hours": round(priced_to_v_not_done_hours, 2),
        "unpriced_units": unpriced_units,
        "unpriced_units_pct_of_remaining": round(100*unpriced_units/remaining_total, 1),
        "priced_to_done_projected_hours_range": [round(priced_hours_lo, 2), round(priced_hours_hi, 2)],
        "note": "Three tiers, never blended into one number: (1) priced_to_done_units (buckets A, B, U) carry a measured rate reaching DONE; (2) priced_to_v_not_done_units (bucket C) carries a measured rate reaching V, its own separate endpoint, NOT DONE -- core_rulebook's only bucket-C cycle moved 42/42 units to V, zero to DONE; (3) unpriced_units (buckets D, M, V, X, Z) have ZERO dedicated clearing cycles in either vehicle book -- no measured rate exists to project from (decisions.md §12 L11/L19: measure before projecting, never carry a number that was not derived). A confident population-wide total blending these three tiers, or asserted for the unpriced tier, would itself be the overconfident-thin-sample failure AGENTS.md rule 7 and this package's own Evidence text warn against.",
    },
}
OUT_PATH = os.path.join(REPO, "docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/forward-plan.json")
json.dump(out, open(OUT_PATH, "w"), indent=2)
print("WROTE forward-plan.json")
print("books:", len(books_out))
print("remaining_total:", remaining_total)
print("totals_by_bucket:", totals_by_bucket)
print("unpriced_units:", unpriced_units, f"{100*unpriced_units/remaining_total:.1f}%")
print("priced_hours_range:", round(priced_hours_lo,2), round(priced_hours_hi,2))
