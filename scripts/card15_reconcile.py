#!/usr/bin/env python3
"""SD-32 card 15 (`decisions.md §12b`/`§12c`) — "sum the piles" reconciliation.

WHY THIS EXISTS
----------------
Card 15's acceptance bar is that the census's discovered population, the
inventory's all-units population, and the shape-ledger's not-done population
reconcile to each other with ONE committed command — and that command must
NAME which population each number is (`decisions.md §12c`), never force them
equal (`decisions.md §12b`'s three legitimately-different populations).

This script re-derives all three live (never a hand-copied number), reports
their relationship, and — the actual card-15 acceptance surface — reports
the disposition status of every one of the census's `kind_unenumerable`
units: how many are (B) proven-not-an-object and already excluded/relabeled
in code this cycle, how many are (A) real objects a measurement lane
identified but this integration cycle has NOT yet added to
`docs/work-inventory.json`, and (for one bucket) how many carry an
UNRESOLVED disposition question this cycle explicitly declined to act on.

This is an honest partial report generator, not a claim of closure — see
`artifacts/gate-0-census-closure/15-integration_cycle_receipt.md` for the
narrative and `kanban.md` row 15 for the card's real status.

Usage
-----
    python3 scripts/card15_reconcile.py --output <path.json>
"""

from __future__ import annotations

import argparse
import json
import os
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))

import census_independent as CI  # noqa: E402
import coverage_ledger as CL  # noqa: E402
import shape_ledger as SL  # noqa: E402
import shape_coverage_standing_gate as GATE  # noqa: E402


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--pcgen-root",
        default=os.environ.get("PCGEN_CORPUS_ROOT"),
        help="$PCGEN_CORPUS_ROOT (defaults to the env var)",
    )
    parser.add_argument(
        "--inventory",
        default=os.path.join(REPO_ROOT, "docs", "work-inventory.json"),
    )
    parser.add_argument("--output", help="write the full report as JSON to this path")
    args = parser.parse_args(argv)

    if not args.pcgen_root:
        print("PCGEN_CORPUS_ROOT not set and --pcgen-root not given", file=sys.stderr)
        return 1

    with open(args.inventory, "r", encoding="utf-8") as fh:
        inventory = json.load(fh)

    # -- Population 1: the census's discovered population (live re-walk) --
    book_dirs = CI.discover_book_dirs(args.pcgen_root)
    scope = CI.classify_scope(book_dirs, inventory)
    pathfinder_root = os.path.join(args.pcgen_root, "pathfinder")
    census = CI.count_objects(pathfinder_root, scope.in_scope)
    census_tracked_total = sum(census["counts_by_kind"].values())
    census_unenumerable_total = census["total_kind_unenumerable_units"]

    # -- Population 2: the inventory's all-origins population --
    inventory_total = len(inventory.get("units") or [])
    inventory_by_kind = {}
    for u in inventory.get("units") or []:
        inventory_by_kind[u.get("kind")] = inventory_by_kind.get(u.get("kind"), 0) + 1

    # -- Population 3: the ledger's not-done population --
    not_done = CL.not_done_population(inventory)
    books = {u.get("book") for u in not_done if u.get("book")}
    corpus_index = SL.build_corpus_index(os.path.join(REPO_ROOT, "data", "corpus"), books)
    ledger = SL.build_ledger(not_done, corpus_index)

    # -- Card 15 disposition status of the census's kind_unenumerable set --
    # (B) proven-not-an-object. `ce__sizes.lst` (9 units) is REMOVED from
    # total_kind_unenumerable_units entirely (a `non_object_file`, not
    # counted anywhere). The CATEGORY:Internal bare-marker reroute (40
    # units, post-adjudication -- narrowed from the original lane's blanket
    # 2,614) is still counted in the live total: it only moves bucket, from
    # `class_feature` to `ability_category:Internal` (837 -> 879). Only the
    # 9 actually shrink `total_this_run` below.
    disposed_b_applied = {
        "class_feature_category_internal_bare_marker_reroute": {
            "units": 40,
            "still_counted_in_total_this_run": True,
            "proof": "artifacts/gate-0-census-closure/"
            "15-card-15-category-internal-adjudication-memo.md (decisions.md "
            "§14c item 4) -- only a CATEGORY:Internal row with neither a "
            "content field nor a resolved ABILITY:...|AUTOMATIC| gateway "
            "reroutes; narrowed from the original lane's blanket 2,614 "
            "after per-row adjudication found that blanket rule wrong for "
            "90.7% of the population",
            "applied_in": "scripts/census_independent.py "
            "_row_is_bare_internal_marker + count_objects "
            "row_dependent_class_feature branch",
        },
        "ce__sizes_lst_engine_covered": {
            "units": 9,
            "still_counted_in_total_this_run": False,
            "proof": "artifacts/gate-0-census-closure/15-card-15-other-kinds-memo.md §7b "
            "(src/rules_core/size.rs SizeCategory, 9 variants, byte-identical)",
            "applied_in": "scripts/census_independent.py NON_OBJECT_FILENAME_TOKENS",
        },
        "kit_filename_collision_fixed": {
            "units": 1,
            "still_counted_in_total_this_run": False,
            "proof": "decisions.md §17 -- census's own \"kit\" in b filename check "
            "false-positived on kitsune_races.lst (the race NAME \"Kitsune\" "
            "contains the substring \"kit\"), diverting one real race-kind row "
            "into kind_unenumerable[\"kit\"]. src/bin/v06_work_inventory.rs's "
            "file_kind never had a \"kit\" branch and always resolved this file "
            "to Kind::Race -- narrowed census's check to \"_kits\" in b (the "
            "real filename convention every genuine *_kits.lst file uses) to "
            "match. Not a new kind: no genuine *_kits.lst file contributes any "
            "row under either the old or the new rule (PCGen's STARTPACK: "
            "block format has no row whose own first field lacks a ':').",
            "applied_in": "scripts/census_independent.py _classify_kind_by_filename",
        },
    }
    disposed_b_still_counted_total = sum(
        v["units"] for v in disposed_b_applied.values() if v["still_counted_in_total_this_run"]
    )

    # (A) real objects a measurement lane identified, NOT yet added to the
    # inventory/census "kind" population this cycle:
    pending_a = {
        "class_feature_residual_original": {
            "units": 179,
            "memo": "15-card-15-class-feature-memo.md §3",
            "why_not_applied": "root cause of the drop (likely a pool-membership "
            "dedup step in v06_work_inventory.rs) is explicitly NOT fully pinned "
            "by the memo itself -- adding a blind rescue list risks silently "
            "re-triggering whatever dedup step currently excludes them",
        },
        "class_feature_internal_adjudicated_pending": {
            "units": 2574,
            "memo": "15-card-15-category-internal-adjudication-memo.md "
            "(decisions.md §14c item 4): 2,371 (A) real content/resolved-"
            "gateway rows + 203 proven facets the census conservatively "
            "keeps counted (cross-file resolution not built into the "
            "walker; under-exclude, not over-exclude, per decisions.md §1a)",
            "why_not_applied": "adjudicated this run but not yet enumerated "
            "into docs/work-inventory.json -- requires narrowing "
            "v06_work_inventory.rs's own separate, unconditional "
            "CATEGORY:Internal trap (`is_internal_category` in "
            "enumerate_file) the same way census_independent.py's "
            "row_dependent_class_feature branch was narrowed; that is a "
            "second, independent codepath from the census walker and was "
            "not attempted this cycle -- flagged forward, not silently "
            "skipped",
        },
        "ability_category_disposition_a": {
            "units": 5108,
            "memo": "15-card-15-ability-category-memo.md",
            "why_not_applied": "requires a new Kind::Ability variant across "
            "src/bin/v06_work_inventory.rs (enum, file_kind, enumerate_file, "
            "refine_kind, duplicate-identity handling) -- too large a surface "
            "to land safely in this cycle's remaining budget",
        },
    }
    # (A) already applied: the 15,438 class_feature rows the census walk
    # agrees with docs/work-inventory.json on (physical book/source_file/
    # source_line join, memo §0/§5), PLUS the 149 `skill` units landed THIS
    # cycle (`Kind::Skill`, `src/bin/v06_work_inventory.rs`) -- 170 real
    # `*_skills.lst` rows the census walk finds, minus 21
    # `core_essentials/ce_skills.lst` rows correctly deleted by the
    # pre-existing `decisions.md §16` core_essentials-residual guard (real,
    # re-derived population growth, not a predicate widening -- see
    # `CORE_ESSENTIALS_RESIDUAL_DELETION_CEILING`'s doc comment, raised
    # 117 -> 138 this cycle for exactly this reason). Census still buckets
    # class_feature as kind_unenumerable (not one of census's own
    # `ADDED_KINDS`/`TEN_KINDS` for the matched-15,438 subset -- the
    # unmatched residual above), so that figure is part of `total_this_run`
    # even though the underlying units are fully tracked; `skill` moved OUT
    # of `kind_unenumerable` entirely once `Kind::Skill` landed, so it is
    # NOT part of `total_this_run` (`census_unenumerable_total`) any more --
    # tracked here for the "sum the piles" narrative, excluded from
    # `accounted_total` below to avoid double-subtracting it.
    already_tracked_a = {
        "class_feature_already_in_inventory": {
            "units": 15438,
            "memo": "15-card-15-class-feature-memo.md §0/§5",
            "note": "already a real inventory unit under kind=class_feature; "
            "counted here only because census's bucket model has no "
            "class_feature 'kind' bucket to move it into",
            "counts_toward_total_this_run": True,
        },
        "skill_landed_this_cycle": {
            "units": 149,
            "census_raw_count": 170,
            "core_essentials_residual_deleted": 21,
            "memo": "15-card-15-other-kinds-memo.md §7a; landed via "
            "Kind::Skill this cycle (card-15-enumerate)",
            "note": "moved OUT of census's kind_unenumerable entirely -- "
            "does not count toward total_this_run below",
            "counts_toward_total_this_run": False,
        },
        "other_kinds_landed_this_cycle": {
            "units": 3550,
            "census_raw_count": 3550,
            "core_essentials_residual_deleted": (
                "template/language only -- 30 + 3 = 33 rows, part of the "
                "138 -> 171 CORE_ESSENTIALS_RESIDUAL_DELETION_CEILING raise"
            ),
            "cross_book_duplicates_dropped": (
                "19 -- core_essentials-sourced template rows restating a "
                "book's own native declaration (e.g. \"Aeon\" in both "
                "core_essentials/ce_templates.lst and "
                "bestiary_2/b2_templates_pc.lst); see "
                "drop_core_essentials_native_restatements in "
                "src/bin/v06_work_inventory.rs, decisions.md §17"
            ),
            "memo": "15-card-15-other-kinds-memo.md §1-5; landed via "
            "Kind::Template/Deity/Power/Domain/Language this cycle "
            "(generic-enumeration, decisions.md §17) through "
            "SIMPLE_FILENAME_KINDS -- the generic mechanism, not one Kind "
            "per cycle",
            "note": "moved OUT of census's kind_unenumerable entirely -- "
            "does not count toward total_this_run below. `kit` (1 unit) is "
            "NOT among these: investigation found it was a census filename "
            "false-positive on kitsune_races.lst, not real content -- see "
            "disposed_b_applied.kit_filename_collision_fixed.",
            "counts_toward_total_this_run": False,
        },
    }
    already_tracked_total = sum(
        v["units"] for v in already_tracked_a.values() if v["counts_toward_total_this_run"]
    )
    pending_a_total = sum(v["units"] for v in pending_a.values())

    # (B) real dispositions a measurement lane proposed but NOT yet applied
    # in code (still counted as kind_unenumerable):
    pending_b_unapplied = {
        "ability_category_gateway_picklist_duplicate": {
            "units": 210 + 560 + 8,
            "memo": "15-card-15-ability-category-summary.md",
            "why_not_applied": "no per-row exclusion rule written in "
            "census_independent.py yet; also blocked on the same tension below",
        },
    }
    pending_b_total = sum(v["units"] for v in pending_b_unapplied.values())

    # The class_feature-lane-vs-ability_category-lane CATEGORY:Internal
    # tension `decisions.md §14c` item 4 named is RESOLVED (not just
    # flagged) as of the `category-internal-adjudication` cycle: split
    # 2,371 (A) / 243 (B) -- 203 proven facets + 40 proven inert -- per-row,
    # not by the class_feature lane's original blanket-(B) file-kind rule.
    # Kept here (renamed from `unresolved_tension`) as the settled record,
    # not an open question.
    resolved_tension = {
        "finding": "class_feature-lane (B)-for-all vs ability_category-lane "
        "(A)-for-81.6%, both applied to CATEGORY:Internal rows -- disjoint "
        "populations (bare *abilities*.lst files vs _abilities_class.lst "
        "files specifically), not a contradiction",
        "resolution": "artifacts/gate-0-census-closure/"
        "15-card-15-category-internal-adjudication-memo.md (decisions.md "
        "§14c item 4): 2,371 (A) / 243 (B) of the 2,614 rerouted rows, "
        "per-row proof, not blanket file-kind exclusion",
        "code_fix_applied": "census_independent.py's reroute now excludes "
        "only the 40 provably-bare rows (`_row_is_bare_internal_marker`); "
        "the 203 proven facets stay counted as class_feature "
        "(under-exclude, not over-exclude) -- see "
        "`class_feature_internal_adjudicated_pending` above for the "
        "enumeration status of the 2,574 that stay counted",
    }

    # Units of `total_this_run` accounted for by SOME disposition, applied
    # or not: the 40 Internal-bare-marker rows are STILL counted in the
    # total (relabeled into `ability_category:Internal`, not removed --
    # `disposed_b_still_counted_total`), the 15,438 already-tracked
    # class_feature rows are still counted (see `already_tracked_a` note),
    # `skill` (149 landed + 21 residual-deleted, 170 total) moved OUT of
    # `kind_unenumerable` entirely this cycle and is excluded here, plus
    # every pending (A)/(B) bucket.
    accounted_total = (
        disposed_b_still_counted_total + already_tracked_total + pending_a_total + pending_b_total
    )
    remaining_undisposed = census_unenumerable_total - accounted_total

    report = {
        "populations": {
            "census_tracked_kind_population": {
                "value": census_tracked_total,
                "command": "python3 scripts/census_independent.py --pcgen-root "
                "$PCGEN_CORPUS_ROOT --inventory docs/work-inventory.json --output <path>; "
                "sum(counts_by_kind.values())",
            },
            "census_kind_unenumerable_population": {
                "value": census_unenumerable_total,
                "command": "same run; total_kind_unenumerable_units",
            },
            "inventory_all_units_population": {
                "value": inventory_total,
                "command": "len(json.load(open('docs/work-inventory.json'))['units'])",
                "by_kind": inventory_by_kind,
            },
            "ledger_not_done_population": {
                "value": ledger["population"],
                "command": "scripts/shape_ledger.py (coverage_ledger.not_done_population "
                "-> shape_ledger.build_ledger)",
                "unclassified_count": ledger["unclassified_count"],
                "families": {fid: info["count"] for fid, info in ledger["families"].items()},
            },
        },
        "relationship": (
            "The three populations are legitimately different (decisions.md §12c), "
            "not forced equal: census_tracked_kind_population + "
            "census_kind_unenumerable_population is the FULL corpus walk "
            "(every row the walker can see); inventory_all_units_population is "
            "everything v06_work_inventory.rs currently enumerates (a subset of "
            "the census's tracked-kind population plus units the census's "
            "coarser filename rules miss or split differently); "
            "ledger_not_done_population is the inventory's not-done subset only. "
            "This cycle's job was to shrink census_kind_unenumerable_population "
            "by disposing every one of its units (A) or (B) -- see disposition_status."
        ),
        "disposition_status_of_kind_unenumerable": {
            "total_this_run": census_unenumerable_total,
            "removed_from_total_across_all_card_15_cycles": {
                "ce__sizes_lst": 9,
                "skill_this_cycle": 170,
                "other_kinds_generic_enumeration_cycle": 3550,
                "kit_filename_collision_fixed": 1,
                "note": "27,847 (pre-any-card-15-cycle) - 9 (ce__sizes.lst, "
                "non_object_file) - 170 (skill moved to a real kind, "
                "Kind::Skill) - 3,550 (template/deity/power/domain/language "
                "moved to real kinds, generic-enumeration cycle, "
                "decisions.md §17) - 1 (kit, census filename false-positive "
                "on kitsune_races.lst fixed, folded into the existing race "
                "count) = 24,117 (total_this_run, above). "
                "The CATEGORY:Internal reroute (originally 2,614, narrowed "
                "to 40 by adjudication) never leaves total_this_run at all "
                "-- it only moves bucket (class_feature <-> "
                "ability_category:Internal), so it does not appear in this "
                "subtraction.",
            },
            "b_disposed_applied": {
                "total_still_counted_in_total_this_run": disposed_b_still_counted_total,
                "total_removed_from_total_this_run": sum(
                    v["units"]
                    for v in disposed_b_applied.values()
                    if not v["still_counted_in_total_this_run"]
                ),
                "detail": disposed_b_applied,
            },
            "a_already_tracked_still_counted_in_total": {
                "total": already_tracked_total,
                "detail": already_tracked_a,
            },
            "a_identified_not_yet_integrated": {
                "total": pending_a_total,
                "detail": pending_a,
            },
            "b_proposed_not_yet_applied_in_code": {
                "total": pending_b_total,
                "detail": pending_b_unapplied,
            },
            "category_internal_tension_resolution": resolved_tension,
            "arithmetic_check": {
                "disposed_b_still_counted_plus_already_tracked_plus_pending_a_plus_pending_b": accounted_total,
                "equals_total_this_run": accounted_total == census_unenumerable_total,
                "remaining_undisposed": remaining_undisposed,
                "note": "every unit in total_this_run (24,117) is accounted "
                "for by exactly one row above: 40 (Internal bare-marker "
                "reroute, disposed B, still counted -- moved into "
                "ability_category:Internal) + 15,438 (class_feature, "
                "already tracked) + 179 (class_feature residual, original, "
                "pending A) + 2,574 (class_feature Internal-adjudicated, "
                "pending A) + 5,108 (ability pending-A) + 778 "
                "(ability_category pending-B) = 24,117. `skill` (149), "
                "`template`/`deity`/`power`/`domain`/`language` (3,550, "
                "landed this cycle via generic-enumeration) and `kit` (1, "
                "disposed B -- census filename collision fixed, not real "
                "content) are NOT in this sum: all three moved OUT of "
                "kind_unenumerable entirely (see already_tracked_a and "
                "disposed_b_applied's own `counts_toward_total_this_run`/"
                "`still_counted_in_total_this_run` flags) -- the "
                "27,668 -> 24,117 drop across this cycle IS that movement. "
                "`ability` (5,108) is the only remaining disposition-(A) "
                "new-kind bucket -- landing it needs a per-row A/B split "
                "first (15-card-15-ability-category-memo.md), not the "
                "filename-only mechanism this cycle used.",
            },
        },
        "gate_3_standing_gate_still_passes": None,  # filled by caller/report consumer
    }

    print(json.dumps(report, indent=2))

    if args.output:
        os.makedirs(os.path.dirname(args.output) or ".", exist_ok=True)
        with open(args.output, "w", encoding="utf-8") as fh:
            json.dump(report, fh, indent=2)
            fh.write("\n")

    return 0


if __name__ == "__main__":
    sys.exit(main())
