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
    # (B) proven-not-an-object, applied in code THIS cycle. `ce__sizes.lst`
    # (9 units) was REMOVED from total_kind_unenumerable_units entirely
    # (27,847 -> 27,838); the CATEGORY:Internal reroute (2,614 units) is
    # still counted in the live total (it moved bucket, from `class_feature`
    # to `ability_category:Internal`) -- both are reported here for the
    # narrative, but only the 9 actually change `total_this_run` below.
    disposed_b_applied = {
        "class_feature_category_internal_reroute": {
            "units": 2614,
            "still_counted_in_total_this_run": True,
            "proof": "artifacts/gate-0-census-closure/15-card-15-class-feature-memo.md §2",
            "applied_in": "scripts/census_independent.py _classify_kind_by_filename "
            "+ count_objects row_dependent_class_feature branch",
        },
        "ce__sizes_lst_engine_covered": {
            "units": 9,
            "still_counted_in_total_this_run": False,
            "proof": "artifacts/gate-0-census-closure/15-card-15-other-kinds-memo.md §7b "
            "(src/rules_core/size.rs SizeCategory, 9 variants, byte-identical)",
            "applied_in": "scripts/census_independent.py NON_OBJECT_FILENAME_TOKENS",
        },
    }

    # (A) real objects a measurement lane identified, NOT yet added to the
    # inventory/census "kind" population this cycle:
    pending_a = {
        "class_feature_residual": {
            "units": 179,
            "memo": "15-card-15-class-feature-memo.md §3",
            "why_not_applied": "root cause of the drop (likely a pool-membership "
            "dedup step in v06_work_inventory.rs) is explicitly NOT fully pinned "
            "by the memo itself -- adding a blind rescue list risks silently "
            "re-triggering whatever dedup step currently excludes them",
        },
        "ability_category_disposition_a": {
            "units": 5108,
            "memo": "15-card-15-ability-category-memo.md",
            "why_not_applied": "requires a new Kind::Ability variant across "
            "src/bin/v06_work_inventory.rs (enum, file_kind, enumerate_file, "
            "refine_kind, duplicate-identity handling) -- too large a surface "
            "to land safely in this cycle's remaining budget; see also the "
            "CATEGORY:Internal A/B tension flagged below",
        },
        "other_kinds_disposition_a": {
            "units": 3551,
            "memo": "15-card-15-other-kinds-memo.md §1-6",
            "buckets": {
                "template_row": 2343,
                "deity": 460,
                "power": 421,
                "domain": 183,
                "language": 143,
                "kit": 1,
            },
            "why_not_applied": "each is a new Kind variant, same surface-area "
            "concern as ability_category above",
        },
        "skill_new_kind": {
            "units": 170,
            "memo": "15-card-15-other-kinds-memo.md §7a",
            "why_not_applied": "new Kind::Skill variant, same surface-area "
            "concern as ability_category above",
        },
    }
    # (A) already applied in a PRIOR cycle: the 15,438 class_feature rows
    # the census walk agrees with docs/work-inventory.json on (physical
    # book/source_file/source_line join, memo §0/§5). Census still buckets
    # ALL class_feature rows as kind_unenumerable (it is not one of census's
    # own TEN_KINDS -- `_classify_kind_by_filename` never returns
    # `("kind", "class_feature")`), so this figure is part of
    # `total_this_run` even though the underlying units are fully tracked.
    already_tracked_a = {
        "class_feature_already_in_inventory": {
            "units": 15438,
            "memo": "15-card-15-class-feature-memo.md §0/§5",
            "note": "already a real inventory unit under kind=class_feature; "
            "counted here only because census's bucket model has no "
            "class_feature 'kind' bucket to move it into",
        }
    }
    already_tracked_total = sum(v["units"] for v in already_tracked_a.values())
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

    # A genuine, unresolved cross-lane tension this cycle found and did NOT
    # paper over (`decisions.md §1a`): the class_feature lane's §2 disposes
    # ALL 2,614 CATEGORY:Internal _abilities_class.lst rows as (B) purely on
    # the walker's own existing, content-blind internal_namespace precedent
    # (v06_work_inventory.rs drops every CATEGORY:Internal row unconditionally,
    # regardless of formula content). The ability_category lane's own
    # PER-ROW classifier, run against the *original* 839-unit
    # ability_category:Internal population, found 685/839 (81.6%) carry
    # independent DEFINE/BONUS/DESC/etc content and disposed them (A), not
    # (B). Both lanes cite real evidence; they were never reconciled against
    # each other. This script does NOT resolve it -- it is named here so the
    # next integration cycle inherits a real open question, not a rounded-off
    # one.
    unresolved_tension = {
        "finding": "class_feature-lane (B)-for-all vs ability_category-lane "
        "(A)-for-81.6%, both applied to CATEGORY:Internal rows, never "
        "cross-checked",
        "class_feature_lane_claim": "all 2,614 rerouted rows are (B), citing "
        "the walker's existing unconditional CATEGORY:Internal exclusion rule",
        "ability_category_lane_finding": "685/839 (81.6%) of the ORIGINAL "
        "ability_category:Internal population carry independent formula "
        "content and were disposed (A), not (B)",
        "action_taken_this_cycle": "the census bucket-relabeling fix "
        "(_abilities_class.lst -> ability_category:Internal) was applied "
        "because it matches the walker's own existing, already-shipped "
        "behaviour for every OTHER _abilities_*.lst file -- a consistency "
        "fix, not a fresh content judgement. This script does NOT apply "
        "the ability_category lane's A/B split to any of the 3,453 rows "
        "now in this bucket (839 original + 2,614 rerouted).",
    }

    # Units of `total_this_run` (27,838) accounted for by SOME disposition,
    # applied or not: the 2,614 Internal-reroute rows are still physically
    # present in the live total (they moved bucket, weren't removed), the
    # 15,438 already-tracked class_feature rows are still counted too (see
    # `already_tracked_a` note), plus every pending (A)/(B) bucket.
    still_counted_reroute = disposed_b_applied["class_feature_category_internal_reroute"][
        "units"
    ]
    accounted_total = (
        still_counted_reroute + already_tracked_total + pending_a_total + pending_b_total
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
            "removed_from_total_this_cycle": {
                "ce__sizes_lst": 9,
                "note": "27,847 (pre-cycle) - 9 = 27,838 (total_this_run, above)",
            },
            "b_disposed_applied_this_cycle_still_counted_in_total": {
                "total": still_counted_reroute,
                "detail": {
                    "class_feature_category_internal_reroute": disposed_b_applied[
                        "class_feature_category_internal_reroute"
                    ]
                },
            },
            "a_already_tracked_prior_cycle_still_counted_in_total": {
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
            "unresolved_cross_lane_tension": unresolved_tension,
            "arithmetic_check": {
                "still_counted_reroute_plus_already_tracked_plus_pending_a_plus_pending_b": accounted_total,
                "equals_total_this_run": accounted_total == census_unenumerable_total,
                "remaining_undisposed": remaining_undisposed,
                "note": "every unit in total_this_run (27,838) is accounted for "
                "by exactly one row above: 2,614 (Internal reroute, disposed "
                "by consistency with the walker's own existing rule, tension "
                "flagged) + 15,438 (class_feature, already tracked in a prior "
                "cycle) + 179 (class_feature residual, pending) + 5,886 "
                "(ability_category total: 5,108 pending-A + 778 pending-B) + "
                "3,551 (other-kinds pending-A) + 170 (skill pending-A) = "
                "27,838.",
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
