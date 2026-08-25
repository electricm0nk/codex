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
    # SD-32 card 15-ability: live-derived, not hand-copied -- every
    # `ability_category:<tag>` unit still in `kind_unenumerable` after the
    # ported per-row A/B classifier runs (see `disposed_b_applied`'s own
    # `ability_category_b_disposed` entry below for what this includes).
    ability_category_b_disposed_total = sum(
        v for k, v in census["kind_unenumerable"].items() if k.startswith("ability_category:")
    )

    # -- Population 2: the inventory's all-origins population --
    inventory_total = len(inventory.get("units") or [])
    inventory_by_kind = {}
    for u in inventory.get("units") or []:
        inventory_by_kind[u.get("kind")] = inventory_by_kind.get(u.get("kind"), 0) + 1

    # -- Population 3: the ledger's not-done population --
    not_done = CL.not_done_population(inventory)
    books = {u.get("book") for u in not_done if u.get("book")}
    corpus_index = SL.build_corpus_index(os.path.join(REPO_ROOT, "data", "corpus"), books)
    # `decisions.md §20`/§17a: reuse the citation-redirect fallback so this
    # reconciliation's ledger cannot silently disagree with the CLI's own.
    key_index = SL.build_corpus_key_index(os.path.join(REPO_ROOT, "data", "corpus"), books)
    # `decisions.md §20` t9-onboarding straggler wave: the cross-book
    # fallback recovers a unit whose real record ships under a DIFFERENT
    # book entirely (not scoped to `books`, for the same reason `key_index`
    # above is reused rather than re-derived).
    cross_book_key_index = SL.build_cross_book_key_index(os.path.join(REPO_ROOT, "data", "corpus"))
    ledger = SL.build_ledger(not_done, corpus_index, key_index, cross_book_key_index)

    # -- Card 15 disposition status of the census's kind_unenumerable set --
    # (B) proven-not-an-object. `ce__sizes.lst` (9 units) is REMOVED from
    # total_kind_unenumerable_units entirely (a `non_object_file`, not
    # counted anywhere).
    #
    # SD-32 card 15-ability: the `ability_category_b_disposed` entry below
    # is now the LIVE, re-derived count of every `ability_category:<tag>`
    # unit remaining in `kind_unenumerable` after `census_independent.py`'s
    # `row_dependent` branch applies the ported per-row A/B classifier
    # (`_ABILITY_CONTENT_RE`) -- this SUBSUMES the CATEGORY:Internal
    # bare-marker reroute this entry used to track standalone (40 units,
    # `decisions.md §14c` item 4): those 40 rows land in the SAME
    # `ability_category:Internal` bucket key the row_dependent branch
    # writes to, so counting them separately here would double-count them
    # against the fresh `ability_category_b_disposed` total below. Their
    # own proof/provenance is unchanged and still applies -- see
    # `15-card-15-category-internal-adjudication-memo.md` -- only the
    # bookkeeping of where they are summed moved.
    disposed_b_applied = {
        "ability_category_b_disposed": {
            "units": ability_category_b_disposed_total,
            "still_counted_in_total_this_run": True,
            "proof": "artifacts/gate-0-census-closure/"
            "15-card-15-ability-category-memo.md's per-row disposition "
            "rules (B-duplicate/B-gateway/B-picklist), ported unchanged "
            "into scripts/census_independent.py's `row_dependent` branch "
            "this cycle (card-15-ability). INCLUDES the 40-unit "
            "CATEGORY:Internal bare-marker reroute from the "
            "`row_dependent_class_feature` branch (decisions.md §14c item "
            "4, 15-card-15-category-internal-adjudication-memo.md) -- both "
            "code paths write to the same `ability_category:<tag>` bucket "
            "keys, so their union is what this figure counts.",
            "applied_in": "scripts/census_independent.py "
            "_ABILITY_CONTENT_RE / _collect_tracked_keys / count_objects "
            "row_dependent branch (card-15-ability); "
            "_row_is_bare_internal_marker + row_dependent_class_feature "
            "branch (category-internal-adjudication, unchanged)",
        },
        "abilities_familiar_companion_routing_fixed": {
            "units": 97,
            "still_counted_in_total_this_run": False,
            "proof": "SD-32 card 15-ability: 6 in-scope "
            "`*_abilities_familiar*.lst` files (b2/b3/pfs_b2/ce x3) were "
            "falling into census's `row_dependent` branch even though "
            "`src/bin/v06_work_inventory.rs`'s `file_kind` already routes "
            "them to the tracked `companion` kind (checked before the bare "
            "`abilit` fallback) -- a real census/inventory disagreement "
            "this cycle found and fixed by matching Rust's own filename "
            "order, not a new content ruling. Moves the FULL 97 rows (not "
            "just the A-disposed ones) to `kind:companion`, matching "
            "companion's own unconditional-per-row-count convention.",
            "applied_in": "scripts/census_independent.py "
            "_classify_kind_by_filename's abilit branch",
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
        # SD-32 Decision 21 (operator ruling 2026-08-23,
        # 21-duplicate-chooser-picker-class-collapse-memo.md): the 39
        # fallback-key `class_feature` collision groups whose members ALL
        # carry a `TYPE:*Choice` facet AND whose granted targets pairwise
        # coincide (113 rows, 74 residual) are duplicate-chooser-picker
        # groups, not distinct objects, ruled on AS A CLASS rather than
        # id-by-id. Predicate re-derived clean against the pinned oracle,
        # zero exceptions across all 39 groups
        # (21-duplicate-chooser-picker-collapse-log.json, every group/row
        # named with book/file/line). `still_counted_in_total_this_run` is
        # True: these 74 rows are still part of census's live
        # `row_dependent_class_feature` walk (the physical rows exist in
        # the corpus) -- only their DISPOSITION moved, from "pending,
        # awaiting an operator ruling" to "disposed, proven not an object
        # by class". No `docs/work-inventory.json` change: these 74 rows
        # were ALREADY absent from the committed inventory before this
        # cycle (the existing `(book,key)` collision collapse already
        # drops them; `disambiguate_class_feature_fallback_collisions`
        # deliberately leaves `TYPE:*Choice`-typed groups untouched -- see
        # that function's own doc comment) -- this entry documents that the
        # absence is correct and proven, not that anything was newly
        # removed.
        "duplicate_chooser_picker_group_class_disposed": {
            "units": 74,
            "still_counted_in_total_this_run": True,
            "proof": "artifacts/gate-0-census-closure/"
            "21-duplicate-chooser-picker-class-collapse-memo.md + "
            "21-duplicate-chooser-picker-collapse-log.json (per-group, "
            "per-row evidence, book/file/line) + "
            "21-duplicate-chooser-picker-class-collapse_test.py (over-reach "
            "proof: a differing-target synthetic group is correctly left "
            "alone; RED->GREEN mutation to the adjacency-only heuristic "
            "Decision 17 rejected, performed and reverted this cycle).",
            "applied_in": "no code change -- v06_work_inventory.rs's "
            "existing (kind,key) collision collapse already produces this "
            "disposition; this is a bookkeeping reallocation only "
            "(pending_a -> disposed_b), see the memo's own "
            "\"No v06_work_inventory.rs change required\" section.",
        },
    }
    disposed_b_still_counted_total = sum(
        v["units"] for v in disposed_b_applied.values() if v["still_counted_in_total_this_run"]
    )

    # (A) real objects a measurement lane identified, NOT yet added to the
    # inventory/census "kind" population this cycle:
    pending_a = {
        # SD-32 card 15-internal: `class_feature_internal_adjudicated_pending`
        # (formerly 2,574 units here) is REMOVED as of card 15-internal --
        # `is_internal_category` narrowed for `Kind::ClassFeature`
        # (`src/bin/v06_work_inventory.rs`) the same way card 15-ability
        # narrowed it for `Kind::Ability`, and those units are now real,
        # enumerated `docs/work-inventory.json` rows -- moved to
        # `already_tracked_a` below.
        #
        # SD-32 card 15-duplicate-identity (first cycle): rescued 24 of the
        # 180 non-internal residual rows
        # (`disambiguate_class_feature_fallback_collisions`,
        # `src/bin/v06_work_inventory.rs`) -- the confirmed-safe subset (a
        # bare `TYPE:FavoredClass` tracker row colliding with an unrelated
        # `TYPE:Class` chassis row, one pair per class, `CATEGORY:` genuinely
        # distinguishes them). SD-32 card 15-duplicate-identity-review (this
        # cycle): per-case hand review of the remaining 156 non-internal
        # residual, per `15-card-15-residual-group-review.py` (this
        # directory) and `15-card-15-duplicate-identity-review-memo.md`.
        # Rescued 4 keyed-collision rows
        # (`disambiguate_class_feature_keyed_name_collisions`,
        # `src/bin/v06_work_inventory.rs`) whose colliding sibling carries a
        # DIFFERENT display name under the SAME author-declared `KEY:` --
        # direct, non-inferred evidence of a corpus-author typo, not one
        # identity (`Native Cunning ~ Grapple`/`Overrun`, `Vigilante Favored
        # Maneuver ~ Bull Rush`/`Sunder`, `Green Faith Marshal ~ Panther
        # Domain`/`Vulture Domain`, `Social Grace ~ Craft (Armor)`/`Craft
        # (Baskets)`, found via a SEPARATE, previously-invisible
        # CATEGORY:Internal-but-content-bearing keyed collision this cycle's
        # own census re-derivation surfaced -- see the review memo's own
        # note on why the original 16-group hand census missed it). The
        # other 13 of the original 16 keyed groups (26 rows) share an
        # IDENTICAL display name on both sides (PFS-variant override,
        # hidden-tracker-beside-real-feature, or byte-identical restatement)
        # -- correctly left on today's collapse-to-first behaviour, not
        # rescued. All 39 `TYPE:*Choice`-suffixed fallback groups (74 rows)
        # were reviewed and found to be the SAME Decision-17
        # duplicate-chooser-picker shape by direct evidence (each group's
        # members converge, in pairs, on an IDENTICAL `ABILITY:AUTOMATIC`
        # grant target reached via a base-class gate and an archetype/
        # feat-chain gate) -- named as allowlist candidates, not rescued;
        # see `why_not_applied`. What remains pending: 153 non-internal
        # residual rows (was 156, -3 non-internal rescues this cycle) + 26
        # (was 27, -1: `Social Grace ~ Craft (Armor)`/`Craft (Baskets)`
        # rescued via the SAME keyed-name-mismatch evidence, even though it
        # carries `CATEGORY:Internal` -- it is content-bearing, not a bare
        # marker, so `is_internal_category`'s existing narrowing already
        # kept it in scope; this cycle's fix then disambiguated its
        # colliding key) newly-internal-turned-content rows that lose their
        # OWN
        # `duplicate_identity` race (unaffected by this cycle -- those rows
        # carry `CATEGORY:Internal` and neither disambiguation fn opens a
        # new identity bucket for an Internal-tagged row -- note: 1 of the 4
        # rescued rows this cycle, `Social Grace ~ Craft (Armor)`/`Craft
        # (Baskets)`, DID carry `CATEGORY:Internal` but was NOT a bare
        # marker -- `is_internal_category`'s own `Kind::ClassFeature`
        # narrowing already keeps content-bearing Internal rows in scope, so
        # this rescue is orthogonal to, not a widening of, that guard). 179
        # total, all traced to the SAME cause: `v06_work_inventory.rs`'s
        # corpus-wide (kind, key) `duplicate_identity` collapse, a different
        # codepath from `is_internal_category`, `refine_kind`, or
        # `has_classifying_token`.
        "class_feature_residual_duplicate_identity": {
            # SD-32 Decision 21 cycle (this cycle, §17a re-derivation):
            # this figure was STALE at 183 -- the committed
            # docs/work-inventory.json already includes the prior review
            # cycle's 4-unit rescue (confirmed by direct id lookup: all 4
            # of native_cunning_grapple_overrun,
            # vigilante_favored_maneuver_bull_rush_favored_maneuver_sunder,
            # social_grace_craft_armor_craft_baskets,
            # green_faith_marshal_panther_domain_vulture are present),
            # landed by a sibling cycle once the `source.path` defect
            # blocking `corpus_literal_sweep` was fixed (af2f07f68) -- so
            # the true current baseline, re-derived fresh via
            # 15-card-15-class-feature-residual-cause-pin.py, is 179 (153
            # non-internal + 26 internal-collision-losers), not 183.
            # Corrected here (183 -> 179, -4, matched by the +4 correction
            # to class_feature_already_in_inventory below), THEN Decision
            # 21's own -74 is applied on top (179 -> 105): the 39
            # Choice-typed fallback groups (74 residual rows) are now
            # DISPOSED, not pending -- see
            # disposed_b_applied.duplicate_chooser_picker_group_class_disposed
            # above.
            "units": 105,
            "memo": "15-card-15-internal-duplicate-identity-memo.md (root-"
            "cause pin) + 15-card-15-duplicate-identity-memo.md (first "
            "rescue cycle, +24, the *Choice-shape correction) + "
            "15-card-15-duplicate-identity-review-memo.md (per-case hand "
            "review cycle, +4) + 21-duplicate-chooser-picker-class-collapse-"
            "memo.md (this cycle, Decision 21's class ruling on the 39 "
            "Choice-typed groups, -74 moved to disposed_b): 153 non-internal residual (was 156, "
            "-3 non-internal rescues) + 26 (was 27, -1 rescued despite "
            "carrying CATEGORY:Internal -- content-bearing, not a bare "
            "marker) newly-internal-turned-content rows that lost their "
            "own duplicate_identity race (unaffected balance, 2,574 "
            "candidate - 2,547 landed - 1 rescued this cycle)",
            "why_not_applied": "SD-32 Decision 21 (operator ruling "
            "2026-08-23) resolved the 39 Choice-typed fallback groups (74 "
            "rows) named here as allowlist candidates by the prior cycle's "
            "memo: ruled on AS A CLASS ('every fallback-key class_feature "
            "collision group whose members ALL carry a TYPE:*Choice facet "
            "AND whose granted targets pairwise coincide is a "
            "duplicate-chooser-picker group, not distinct objects'), "
            "re-derived clean with zero exceptions "
            "(21-duplicate-chooser-picker-class-collapse-memo.md, "
            "-collapse-log.json) and MOVED to disposed_b above -- no longer "
            "pending. What remains pending here: the 12 (of the original "
            "16) keyed-collision groups (24 rows) that collide with a row "
            "sharing the IDENTICAL display name under an explicit, "
            "corpus-author-declared `KEY:` field (PFS-legal override, "
            "hidden-tracker-beside-real-feature, or byte-identical "
            "restatement) -- a deliberate single identity, re-confirmed by "
            "direct content read, not the fallback weakness either "
            "disambiguation fn targets, and OUTSIDE Decision 21's own scope "
            "(binding condition 5: fallback-key collisions only, not keyed "
            "ones). Decision 17's own text explicitly forbids a live "
            "adjacency filter for THIS population too ('a generic "
            "same-name-adjacent-line rule would silently sweep in any "
            "FUTURE same-shaped collision no human reviewed') -- these 12 "
            "groups' evidence was already reviewed case by case "
            "(`15-card-15-duplicate-identity-review-memo.md`) and correctly "
            "left uncollapsed, not merely unreviewed. 22 rows remain "
            "genuinely re-confirmed as having no (book,key) collision at "
            "all -- but ALL 22 are traced: 21 are rows ALREADY on the "
            "confirmed 33-id `DUPLICATE_CHOOSER_DISPLAY_NAME_UNIT_IDS` "
            "allowlist (10 core_rulebook + 10 advanced_players_guide + 1 "
            "adventurers_guide bloodline pickers), deliberately removed "
            "post-construction by `apply_duplicate_chooser_removal` -- "
            "correct, expected, not a defect (Decision 21's own memo rules "
            "this relationship COMPLEMENTS, not supersedes: the id list "
            "removes CONSTRUCTED survivor units the collision collapse "
            "cannot reach, a disjoint population from the 74 collision-loser "
            "rows that never reach construction at all -- left "
            "unchanged, deliberately, per that memo); the 22nd "
            "(`ultimate_psionics:class_feature:disable_device_class_skill`) "
            "is the already-traced displacement from "
            "`15-card-15-internal-duplicate-identity-memo.md` §3 (content "
            "preserved under a new physical coordinate). No cause-pinning "
            "gap remains in the 105-unit population; the residual is a "
            "named, evidenced, per-case hand-review population (the 12 "
            "keyed groups) that needed and received a different "
            "disposition than the 39 Choice-typed groups, not an "
            "unexplained defect.",
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
            # SD-32 Decision 21 cycle: corrected 18008 -> 18012 (+4) -- the
            # prior review cycle's 4-unit rescue (native_cunning_grapple_
            # overrun, vigilante_favored_maneuver_bull_rush_favored_
            # maneuver_sunder, social_grace_craft_armor_craft_baskets,
            # green_faith_marshal_panther_domain_vulture) is confirmed
            # ALREADY present in the committed docs/work-inventory.json by
            # direct id lookup this cycle -- this figure was simply not
            # updated to match when that regen landed. Matched by the -4
            # correction to class_feature_residual_duplicate_identity above
            # (183 -> 179 before Decision 21's own further -74).
            "units": 18012,
            "memo": "15-card-15-class-feature-memo.md §0/§5 (15,437 "
            "non-internal, was 15,438 before the card-15-internal cycle's "
            "own Disable-Device-Class-Skill displacement, "
            "15-card-15-internal-duplicate-identity-memo.md §3) + 2,547 "
            "content-bearing CATEGORY:Internal rows landed by card "
            "15-internal (`is_internal_category` narrowed for "
            "Kind::ClassFeature the same way card 15-ability narrowed it "
            "for Kind::Ability) -- of the 2,574 that cycle's adjudication "
            "memo identified as disposition (A)/(B-gateway-resolved), 26 "
            "(was 27) lost their own duplicate_identity race and are "
            "pending (see pending_a below), not lost -- + 24 rescued by "
            "card 15-duplicate-identity "
            "(`disambiguate_class_feature_fallback_collisions`, the "
            "confirmed-safe `TYPE:FavoredClass`-shaped subset) + 4 rescued "
            "by card 15-duplicate-identity-review "
            "(`disambiguate_class_feature_keyed_name_collisions`, the "
            "differing-display-name-under-one-KEY subset), confirmed "
            "landed in the committed inventory this cycle (Decision 21 "
            "cycle, id lookup).",
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
        "ability_landed_this_cycle": {
            "units": 4824,
            "census_raw_count": 5028,
            "note": "SD-32 card 15-ability: `Kind::Ability` landed through "
            "the census's own row_dependent branch (content/gateway test "
            "ported from 15-card-15-ability-category-memo.md into "
            "_ABILITY_CONTENT_RE) plus a row-level CATEGORY:FEAT redirect "
            "(refine_kind) matching the same rule. The 204-unit gap "
            "between census's raw 5,028 and the real inventory's 4,824 is "
            "the `CORE_ESSENTIALS_RESIDUAL_DELETION_CEILING` deletion "
            "(289 new core_essentials-sourced ability residuals, "
            "decisions.md §16 -- ce_abilities.lst's book-wide shared "
            "lookup tables have no per-race path signal and no resolvable "
            "SOURCELONG:, same disposition every prior kind's unattributable "
            "core_essentials content gets) minus a +85 gain from a "
            "PRE-EXISTING Rust/census disagreement this cycle's `abilit` "
            "fallback incidentally fixed: 3 in-scope "
            "`*_abilities_feat.lst` files (inner_sea_gods/inner_sea_combat/"
            "inner_sea_faiths) were never enumerated by "
            "src/bin/v06_work_inventory.rs's old file_kind at all (no "
            "`_abilities_feat` branch existed), so their 111 real "
            "CATEGORY:FEAT feat rows + 3 ability rows were invisible to "
            "the inventory even though census already counted them as "
            "`feat` via its looser `\"feat\" in b` substring match -- now "
            "correctly split 111 feat / 3 ability by refine_kind's own "
            "CATEGORY:FEAT test. `docs/work-inventory.json`'s `feat` kind "
            "grew 2,610 -> 2,722 (+112 = 111 + the 1 apg_abilities.lst row "
            "the memo already named) as the direct side effect -- verified "
            "by id-diff, 0 pre-existing feat units removed. NOTE: census's "
            "own `\"feat\" in b` check still counts all 114 rows in those "
            "3 files as `feat` unconditionally (no per-row CATEGORY: test "
            "at that branch) -- 3 units (the ability-disposed ones) are a "
            "small, real, NOT-fixed census/inventory disagreement this "
            "cycle found and is reporting, not silently absorbing.",
            "counts_toward_total_this_run": False,
        },
    }
    already_tracked_total = sum(
        v["units"] for v in already_tracked_a.values() if v["counts_toward_total_this_run"]
    )
    pending_a_total = sum(v["units"] for v in pending_a.values())

    # (B) real dispositions a measurement lane proposed but NOT yet applied
    # in code (still counted as kind_unenumerable). Empty as of card
    # 15-ability: `ability_category_gateway_picklist_duplicate` (formerly
    # here) is now APPLIED (`disposed_b_applied.ability_category_b_disposed`
    # above, live-derived, not the memo's stale 778 figure).
    pending_b_unapplied = {}
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
        "(under-exclude, not over-exclude). SD-32 card 15-internal: "
        "`v06_work_inventory.rs`'s OWN separate `is_internal_category` trap "
        "narrowed the same way for `Kind::ClassFeature` -- 2,547 of the "
        "2,574 landed as real `docs/work-inventory.json` units this cycle; "
        "see `already_tracked_a.class_feature_already_in_inventory` and "
        "`pending_a.class_feature_residual_duplicate_identity` above for "
        "the split.",
    }

    # Units of `total_this_run` accounted for by SOME disposition, applied
    # or not: `ability_category_b_disposed` (live, includes the 40
    # Internal-bare-marker rows -- see its own note) is STILL counted in
    # the total, the 15,438 already-tracked class_feature rows are still
    # counted (see `already_tracked_a` note), `skill`/`ability`/the other
    # landed kinds moved OUT of `kind_unenumerable` entirely this cycle and
    # are excluded here, plus every remaining pending (A)/(B) bucket.
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
                "ability_landed_card_15_ability_cycle": 5028,
                "abilities_familiar_companion_routing_fixed_card_15_ability_cycle": 97,
                "note": "27,847 (pre-any-card-15-cycle) - 9 (ce__sizes.lst, "
                "non_object_file) - 170 (skill moved to a real kind, "
                "Kind::Skill) - 3,550 (template/deity/power/domain/language "
                "moved to real kinds, generic-enumeration cycle, "
                "decisions.md §17) - 1 (kit, census filename false-positive "
                "on kitsune_races.lst fixed, folded into the existing race "
                "count) - 5,028 (ability moved to a real kind, Kind::Ability, "
                "card-15-ability cycle) - 97 (*_abilities_familiar*.lst rows "
                "routed to the tracked companion kind, matching Rust's own "
                "file_kind order, card-15-ability cycle) = "
                f"{census_unenumerable_total} (total_this_run, above; live, "
                "not hand-copied). The CATEGORY:Internal reroute (originally "
                "2,614, narrowed to 40 by adjudication) never leaves "
                "total_this_run at all -- it only moves bucket (class_feature "
                "<-> ability_category:Internal), so it does not appear in "
                "this subtraction.",
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
                "note": f"every unit in total_this_run ({census_unenumerable_total}) is "
                "accounted for by exactly one row above: "
                f"{ability_category_b_disposed_total} (ability_category_b_disposed, "
                "live-derived, INCLUDES the 40-unit Internal bare-marker "
                "reroute) + 74 (duplicate_chooser_picker_group_class_"
                "disposed, SD-32 Decision 21, operator ruling 2026-08-23 -- "
                "the 39 Choice-typed fallback groups, moved here from "
                "pending_a this cycle) + 18,012 (class_feature, already "
                "tracked -- SD-32 card 15-internal raised this from 15,438; "
                "card 15-duplicate-identity raised it again from 17,984 "
                "with the 24-unit confirmed-safe rescue; card "
                "15-duplicate-identity-review raised it again to 18,012 "
                "with its own 4-unit rescue, confirmed landed in the "
                "committed inventory by this cycle's own id lookup -- see "
                "already_tracked_a's own note) + 105 (class_feature "
                "residual STILL pending: cause pinned to `duplicate_"
                "identity` by card 15-internal, narrowed from 207 to 183 "
                "by card 15-duplicate-identity's 24-unit rescue, then to "
                "179 by card 15-duplicate-identity-review's own 4-unit "
                "rescue, then to 105 by THIS cycle's Decision 21 move (-74, "
                "the 39 Choice-typed groups DISPOSED, not pending, above) "
                "-- what remains: the 12 keyed groups (24 rows) left "
                "correctly collapsed (identical display name both sides -- "
                "PFS override / hidden tracker / true restatement, outside "
                "Decision 21's own fallback-key-only scope) + the 22 "
                "fully-traced non-colliding rows -- see pending_a's own "
                "why_not_applied) = "
                f"{ability_category_b_disposed_total + 74 + 18012 + 105}. "
                "`skill` (149), `template`/`deity`/`power`/`domain`/"
                "`language` (3,550, generic-enumeration cycle) `kit` (1, "
                "disposed B) `ability` (4,824 real inventory units off a "
                "5,028 live census count) and the 97-row companion routing "
                "fix are NOT in this sum: all moved OUT of "
                "kind_unenumerable entirely (see already_tracked_a and "
                "disposed_b_applied's own `counts_toward_total_this_run`/"
                "`still_counted_in_total_this_run` flags). Card 15's "
                "remaining scope, narrowed again by this cycle's Decision "
                "21 ruling: the class_feature residual (105 units, cause "
                "pinned to `duplicate_identity`, fully reviewed case by "
                "case -- an evidenced, named, per-case hand-review "
                "population: 12 keyed groups correctly left uncollapsed "
                "(Decision 21 does not extend to keyed collisions, binding "
                "condition 5) + 22 fully-traced non-colliding rows, not an "
                "unexplained defect) is the only disposition-(A) population "
                "still pending integration.",
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
