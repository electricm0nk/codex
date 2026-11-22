"""Re-derives artifacts/epic-5-forward-plan/power-table-cost.json (AT-34-E5-003).

Run from anywhere inside the repo:
    python3 docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/build_power_table_cost.py

Evidence bar (epic-breakdown.md, AT-34-E5-003): "421 units, all inside `ultimate_psionics` --
not built here, costed here, using the measured build rate from Epic 2's eight tables and the
spread across them. Evidence: the projected cost, the rate it derives from, and the reason it
was not built (decisions.md §7). Plus what `ultimate_psionics` would still need after it exists
-- that book has all eight non-DONE buckets occupied, so the table alone does not close it, and
the plan must say so."

This artifact is read-only against the rest of the repo (workflow-instruction.md §3): it prices
the `power` table, it does not build it. `docs/work-inventory.json`, `missing-engine-tables.json`
and `table-build-rate.json` are all read here, never written.
"""
import json, subprocess, sys, os, collections

REPO = subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True, text=True, check=True).stdout.strip()
sys.path.insert(0, os.path.join(REPO, "scripts"))
import completion_atlas as ca

HEAD = subprocess.run(["git", "rev-parse", "HEAD"], cwd=REPO, capture_output=True, text=True, check=True).stdout.strip()

inv = json.load(open(f"{REPO}/docs/work-inventory.json"))
units = inv["units"]

pres_all = ca.partition(units)
assert not pres_all["unclassified_ids"] and not pres_all["overlap_ids"], "live atlas must be clean before pricing power"
assert pres_all["examined"] == 49438, pres_all["examined"]

# --- population: power kind, live, cross-checked two independent ways ---------------------
missing_tables_path = os.path.join(REPO, "docs/release/SD-34-book-completion/artifacts/epic-1-atlas/missing-engine-tables.json")
missing_tables = json.load(open(missing_tables_path))
power_from_atlas_artifact = missing_tables["kinds"]["power"]["count"]
power_book = list(missing_tables["kinds"]["power"]["by_book"].keys())[0]
assert len(missing_tables["kinds"]["power"]["by_book"]) == 1, "power was scoped to exactly one book at authoring time"

a_units_power = [
    u for u in units
    if u.get("status") == "engine-does-not-hold" and "has_no_engine_table" in (u.get("evidence") or "") and u.get("kind") == "power"
]
power_live = len(a_units_power)
assert power_live == power_from_atlas_artifact == 421, (power_live, power_from_atlas_artifact)
assert all(u["book"] == power_book for u in a_units_power), "power must be single-book, per epic-breakdown.md's own claim"

# Third cross-check: the corpus directory itself.
power_dir = os.path.join(REPO, "data/corpus", power_book, "power")
power_file_count = len([f for f in os.listdir(power_dir) if f.endswith(".json")])
assert power_file_count == power_live == 421, (power_file_count, power_live)

# Capability register (AT-34-E5-002) already named this population -- confirm agreement rather
# than re-deriving a fourth time with a different method.
cap_register_path = os.path.join(REPO, "docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/capability-register.json")
cap_register = json.load(open(cap_register_path))
power_cap_row = next(c for c in cap_register["capabilities"] if c["id"] == "power_engine_table")
assert power_cap_row["population"] == power_live, (power_cap_row["population"], power_live)

# --- directory-name-match check (table-build-rate.json's own stated cost driver) -----------
# `kind_dir_for` only resolves the 7 kinds Epic 2 built; power is not one of them, so this
# checks the ACTUAL corpus layout directly: does data/corpus/<book>/power/ (the kind name)
# exist and hold every one of power's 421 records, the same way ability/template/deity/domain/
# skill/language's directories match their kind name (only `trait` -- `trait_generic` --
# needed an override).
directory_matches_kind_name = os.path.isdir(power_dir)
assert directory_matches_kind_name, "power's directory-match assumption must hold against the live tree"

# --- rate derivation from table-build-rate.json ---------------------------------------------
rate_path = os.path.join(REPO, "docs/release/SD-34-book-completion/artifacts/epic-2-tables/table-build-rate.json")
rate = json.load(open(rate_path))
tables_by_kind = {t["kind"]: t for t in rate["tables"]}

# The kinds whose corpus directory matched their kind name exactly (no override needed) --
# power is one of these by directory shape, not one of `trait` (mismatch, dearer) or
# `companion` (pre-existing table, not a new build).
matched_dir_kinds = ["ability", "template", "deity", "domain", "skill", "language"]
assert set(matched_dir_kinds) <= set(tables_by_kind.keys())

matched_marginal_lines = [tables_by_kind[k]["marginal_lines_measured"] for k in matched_dir_kinds]
matched_wall_seconds = [tables_by_kind[k]["est_wall_time_seconds"] for k in matched_dir_kinds]

trait_marginal_lines = tables_by_kind["trait"]["marginal_lines_measured"]
trait_wall_seconds = tables_by_kind["trait"]["est_wall_time_seconds"]

projected_marginal_lines_low = min(matched_marginal_lines)
projected_marginal_lines_high = max(matched_marginal_lines)
projected_wall_seconds_low = min(matched_wall_seconds)
projected_wall_seconds_high = max(matched_wall_seconds)

# --- what ultimate_psionics still needs after `power` exists --------------------------------
pres_book = ca.partition(units, book=power_book)
book_counts = dict(pres_book["counts"])
assert book_counts.get("A", 0) == power_live, "bucket A in this book must equal power's population -- power is its ONLY bucket-A kind"

occupied_non_done_before = sorted(b for b in ca.BUCKET_ORDER if b not in ("DONE",) and book_counts.get(b, 0) > 0)
occupied_non_done_after = sorted(b for b in occupied_non_done_before if b != "A")

out = {
    "criterion": "AT-34-E5-003",
    "generated_at_head": HEAD,
    "re_derive_command": "python3 docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/build_power_table_cost.py",
    "population": {
        "kind": "power",
        "book": power_book,
        "count": power_live,
        "cross_checks": [
            "docs/work-inventory.json: status==engine-does-not-hold AND evidence contains 'has_no_engine_table' AND kind=='power'",
            "artifacts/epic-1-atlas/missing-engine-tables.json: kinds.power.count",
            f"data/corpus/{power_book}/power/*.json file count (live directory listing)",
            "artifacts/epic-5-forward-plan/capability-register.json: capabilities[power_engine_table].population",
        ],
        "note": "all four independent sources agree at 421 -- single book, single kind, matches "
                "epic-breakdown.md's own population statement exactly.",
    },
    "reason_not_built": {
        "ruling": "decisions.md §7",
        "summary": "power's 421 units all sit inside a 3,498-unit book (ultimate_psionics) that "
                   "occupies 7 other non-DONE buckets besides A. Building the table would clear "
                   "bucket A but bank no CLOSED BOOK to prove it, so it is priced here for the "
                   "successor bundle's cleanest opening move instead of built inside SD-34's own "
                   "two-book scope (core_rulebook, ultimate_campaign).",
    },
    "rate_derivation": {
        "source": "artifacts/epic-2-tables/table-build-rate.json (AT-34-E2-003)",
        "method": "table-build-rate.json's own finding: marginal cost is dominated by whether "
                  "the kind's corpus directory name matches its kind name, not by record count "
                  "(ability, 4337 records, and domain, 183 records, cost nearly the same because "
                  "both use the unmodified generic loader; trait's mismatched directory, "
                  "trait_generic, is the one dearer case). power's directory "
                  f"(data/corpus/{power_book}/power/) matches its kind name exactly -- confirmed "
                  "live above -- so it is priced against the 6 kinds that also matched "
                  "(ability, template, deity, domain, skill, language), NOT against trait's "
                  "dearer, mismatched-directory tier.",
        "matched_directory_kinds": matched_dir_kinds,
        "matched_kinds_marginal_lines": dict(zip(matched_dir_kinds, matched_marginal_lines)),
        "matched_kinds_est_wall_seconds": dict(zip(matched_dir_kinds, matched_wall_seconds)),
        "excluded_comparators": {
            "trait": {
                "reason": "directory mismatch (trait_generic vs trait) -- the one dearer tier; "
                          "power's directory matches, so this tier is excluded as the wrong shape.",
                "marginal_lines_measured": trait_marginal_lines,
                "est_wall_time_seconds": trait_wall_seconds,
            },
            "companion": {
                "reason": "pre-existing table (SD-29) -- this bundle's only companion cost was a "
                          "fail-closed proof, not a table build; not a build-cost comparator at all.",
            },
        },
        "projected_marginal_lines_range": [projected_marginal_lines_low, projected_marginal_lines_high],
        "projected_wall_time_seconds_range": [projected_wall_seconds_low, projected_wall_seconds_high],
        "confidence_note": "DOUBLE-ESTIMATE, marked per AGENTS.md rule 9: table-build-rate.json's "
                           "own per-table wall times are already pro-rated ESTIMATEs (no table was "
                           "independently stopwatched -- all 7 new kinds landed in one shared-loader "
                           "commit), not independent measurements. This range projects power onto "
                           "that same estimate, one level removed from a real timing. It is NOT a "
                           "point estimate -- reporting the range domain/skill/language..ability/"
                           "template/deity's own spread produced is honest; collapsing it to one "
                           "number would fabricate precision the underlying data does not carry.",
    },
    "book_still_needs_after_power_exists": {
        "book": power_book,
        "book_total_units": pres_book["examined"],
        "live_bucket_counts": book_counts,
        "occupied_non_done_buckets_before": occupied_non_done_before,
        "occupied_non_done_buckets_after_power": occupied_non_done_after,
        "instrument_correction_note": "decisions.md §7 (authored earlier in this bundle) states "
                                      "ultimate_psionics has 'all eight non-DONE buckets occupied' "
                                      "(A=852, B=769, C=304, D=356, M=168, V=322, U=10 -- summing "
                                      "to 2,781, not matching live bucket B/M's current split). "
                                      "Re-derived live at HEAD (decisions.md §12 L2 -- never carry "
                                      "a number forward), this book occupies 7 non-DONE buckets "
                                      "(A, B, C, D, M, U, V), not 8 -- neither X nor Z has any "
                                      "unit here. The bucket split has moved since §7 was written "
                                      "(A/B/M's live counts differ from the cited ones) though the "
                                      "book's total (3,498) and power's population (421) have not. "
                                      "Building `power` clears bucket A to 0 and leaves 6 non-DONE "
                                      "buckets still occupied (B, C, D, M, U, V) -- the table alone "
                                      "does not close the book, matching the criterion's own bar.",
    },
}

OUT_PATH = os.path.join(REPO, "docs/release/SD-34-book-completion/artifacts/epic-5-forward-plan/power-table-cost.json")
json.dump(out, open(OUT_PATH, "w"), indent=2)
print("WROTE power-table-cost.json")
print("power population:", power_live, "book:", power_book)
print("projected_marginal_lines_range:", [projected_marginal_lines_low, projected_marginal_lines_high])
print("projected_wall_time_seconds_range:", [projected_wall_seconds_low, projected_wall_seconds_high])
print("occupied_non_done_buckets_before:", occupied_non_done_before)
print("occupied_non_done_buckets_after_power:", occupied_non_done_after)
