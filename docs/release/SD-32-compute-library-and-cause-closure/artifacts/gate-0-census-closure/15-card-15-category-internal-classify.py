#!/usr/bin/env python3
"""Card 15 (`census-scope-closure`, `decisions.md` §14c item 4) -- per-row
disposition classifier for the 2,614 `CATEGORY:Internal` rows found in
`_abilities_class.lst` files (the `row_dependent_class_feature` bucket of
`scripts/census_independent.py`, currently all rerouted to the single label
`ability_category:Internal` without a per-row (A)/(B) test).

WHY THIS EXISTS
----------------
Two SD-32 lanes reached opposite conclusions on this population:
  * the `class_feature` lane disposed all 2,614 as (B) "not an object" by
    class-wide analogy to `census_independent.py`'s existing exclusion for
    *other* `_abilities_*.lst` files -- but never applied a per-row content
    test to this population itself (`15-card-15-class-feature-memo.md` §2).
  * the sibling `ability_category` lane built and ran a real per-row
    classifier (`has_content` / `has_gateway` / `has_duplicate`) on the
    *other* 839 `ability_category:Internal` rows (from bare `*abilities*.lst`
    files) and found 81.6% disposition (A).

This script applies that same classifier -- reused via the same
`census_independent` primitives, not reimplemented -- directly to the 2,614
`_abilities_class.lst` `CATEGORY:Internal` rows, so the two lanes' verdicts
are produced by one comparable method instead of two different tests on two
different populations.

Content-token list is the ability_category classifier's list, extended with
`SPELLKNOWN` and `TEMPBONUS` (the two token families the Opus adversarial
verifier named that the original ability_category classifier's regex did not
cover -- `decisions.md §14c` item 4). Both versions of the count are reported
so the effect of the extension is visible, not silently substituted.

Also runs the parent-resolution test §12b requires for a (B) disposition:
a gateway row's `ABILITY:...|AUTOMATIC|<target>` must resolve to a `KEY:`
that is *actually* counted somewhere else (a tracked kind, the already-
tracked 15,438/179 `class_feature` units, or an `ability_category:*`
disposition-A row) -- not merely assumed to.

USAGE
-----
    python3 15-card-15-category-internal-classify.py \
        --repo-root <repo> \
        --corpus-root <repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data \
        --inventory <repo>/docs/work-inventory.json \
        --ability-category-rows <repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/15-card-15-ability-category-rows.jsonl \
        --diff-json <repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/diff.json \
        --output-jsonl <out>.jsonl --output-summary-md <out>.md

Exits non-zero if the row count it derives does not match the CATEGORY:Internal
population found by walking `_abilities_class.lst` files independently of
`census_independent.py`'s own bucket label (Decision 1a: fail closed).
"""
from __future__ import annotations

import argparse
import collections
import json
import os
import re
import sys


def _load_census_module(repo_root: str):
    scripts_dir = os.path.join(repo_root, "scripts")
    sys.path.insert(0, scripts_dir)
    import census_independent as ci  # noqa: PLC0415

    return ci


# Base content-token set -- identical to the ability_category classifier
# (`15-card-15-ability-category-classify.py`), for direct comparability.
CONTENT_RE_BASE = re.compile(
    r"DEFINE:|BONUS[A-Z]*:|DESC:|ASPECT:|CSKILL:|MOVE:|AUTO:|TEMPLATE:|SPROP:|QUALITY:|SR:|DR:|SAB:|VISION:"
)
# Extended set -- adds the two token families the Opus verifier named that
# the base regex does not cover. Both counts are reported (see module
# docstring) so the effect of the extension is visible, not silently
# substituted for the base method.
CONTENT_RE_EXT = re.compile(
    r"DEFINE:|BONUS[A-Z]*:|DESC:|ASPECT:|CSKILL:|MOVE:|AUTO:|TEMPLATE:|SPROP:|QUALITY:|SR:|DR:|SAB:|VISION:|SPELLKNOWN[A-Z]*:|TEMPBONUS:"
)
# Full set -- the extended set widened again after a whole-record field
# inventory over all 2,614 rows found real application-governing fields the
# extended set still missed (AGENTS.md concurrency rule: "a grep filtered to
# BONUS/PRE hides STACK/MULT and other application-governing fields" --
# CHOOSE:/NATURALATTACKS:/COMPANIONLIST:/ADD:/FOLLOWERS:/UDAM:/UMULT:/
# SELECT:/COST:/MOVECLONE:/SPELLS:/SERVESAS:/DEFINESTAT:/UNENCUMBEREDMOVE:/
# BENEFIT: all showed up carrying real mechanical or narrative payload on
# rows the extended set alone would have called bare picklist entries).
# This is the disposition-of-record test; base/extended are retained only
# for the reconciliation table.
CONTENT_RE_FULL = re.compile(
    r"DEFINE:|BONUS[A-Z]*:|DESC:|ASPECT:|CSKILL:|MOVE:|AUTO:|TEMPLATE:|SPROP:|QUALITY:|SR:|DR:|SAB:|VISION:|"
    r"SPELLKNOWN[A-Z]*:|TEMPBONUS:|CHOOSE:|NATURALATTACKS:|COMPANIONLIST:|ADD:|FOLLOWERS:|UDAM:|UMULT:|"
    r"SELECT:|COST:|MOVECLONE:|SPELLS:|SERVESAS:|DEFINESTAT:|UNENCUMBEREDMOVE:|BENEFIT:|SPELLLEVEL:|"
    r"CMB:|UNENCUMBEREDMOVE:"
)
# Narrowest test -- the class_feature memo's own standard (DEFINE/BONUS only,
# i.e. shape_ledger's formula-extraction field list). Reported for
# reconciliation against the memo's "both classify as F0" framing.
FORMULA_RE = re.compile(r"DEFINE:|BONUS[A-Z]*:")

GATEWAY_RE = re.compile(r"ABILITY:[^\t]+\|AUTOMATIC\|")
GATEWAY_TARGET_RE = re.compile(r"ABILITY:[^\t]+\|AUTOMATIC\|([^\t|]+)")


def _key_field(line: str):
    for f in line.split("\t"):
        f = f.strip()
        if f.upper().startswith("KEY:"):
            return f.split(":", 1)[1].strip()
    return None


def main(argv=None) -> int:
    p = argparse.ArgumentParser()
    p.add_argument("--repo-root", default=os.getcwd())
    p.add_argument("--corpus-root", required=True)
    p.add_argument("--inventory", required=True)
    p.add_argument("--ability-category-rows", required=True)
    p.add_argument("--diff-json", required=True)
    p.add_argument("--output-jsonl", required=True)
    p.add_argument("--output-summary-md", required=True)
    args = p.parse_args(argv)

    ci = _load_census_module(args.repo_root)

    with open(args.inventory, "r", encoding="utf-8") as fh:
        inventory = json.load(fh)

    pathfinder_root = os.path.join(args.corpus_root, "pathfinder")
    scope = ci.classify_scope(ci.discover_book_dirs(args.corpus_root), inventory)

    # ------------------------------------------------------------------
    # Pass 1: build the resolvable-target universe for the parent-
    # resolution test -- every KEY: already counted somewhere:
    #   (a) the ten tracked kinds (feat/class/spell/monster/monster_ability/
    #       equipment/equipment_modifier/companion/race/race_trait), by
    #       walking the corpus the same way census_independent.py counts them;
    #   (b) work-inventory.json's currently-tracked class_feature units
    #       (corpus_key field -- the 15,438/15,439 side of the class_feature
    #       memo's reconciliation);
    #   (c) the ability_category lane's own disposition-A rows (5,108 units,
    #       from the sibling lane's committed output) -- a class_feature
    #       CATEGORY:Internal gateway row may target a Special-Ability-
    #       category row the sibling lane already counted as (A).
    # ------------------------------------------------------------------
    # Resolution target = KEY: field when the row declares one, else the
    # row's own bare identity (first tab-separated column) -- this is
    # PCGen's own resolution rule for an ABILITY:...|AUTOMATIC|<target>
    # reference (most feats and many class-feature-adjacent rows carry no
    # explicit KEY: field at all and are referenced by bare name; verified
    # against the corpus for e.g. `cr_feats.lst`'s "Double Slice", which has
    # no KEY: field). The ability_category classifier's own KEY:-only rule
    # was scoped to its *cross-kind duplicate* check (the "shared-name
    # hazard": two DIFFERENT kinds' rows that happen to share a display
    # name); resolving a gateway's own target against the SAME kind's own
    # identity/KEY population is a different, narrower join and does not
    # reintroduce that hazard.
    tracked_keys: dict[str, set[str]] = collections.defaultdict(set)

    for bd in scope.in_scope:
        book_dir = os.path.join(pathfinder_root, bd.rel_path)
        for dirpath, _dirnames, filenames in os.walk(book_dir):
            for fn in sorted(filenames):
                if not fn.lower().endswith(".lst"):
                    continue
                full = os.path.join(dirpath, fn)
                bucket, key = ci._classify_kind_by_filename(fn, bd.book_id)
                if bucket == "row_dependent":
                    for identity, raw in ci._parse_lst_rows(full):
                        cat = ci._row_category_tag(raw)
                        if cat and cat.upper() == "FEAT":
                            ident_upper = identity.upper()
                            if ident_upper.endswith((".FORGET", ".MOD")):
                                continue
                            kf = _key_field(raw) or identity
                            tracked_keys["feat"].add(kf)
                    continue
                if bucket != "kind":
                    continue
                for identity, raw in ci._parse_lst_rows(full):
                    ident_upper = identity.upper()
                    if ident_upper.endswith((".FORGET", ".MOD")):
                        continue
                    kf = _key_field(raw) or identity
                    tracked_keys[key].add(kf)

    for u in inventory["units"]:
        if u.get("kind") == "class_feature" and u.get("corpus_key"):
            tracked_keys["class_feature"].add(u["corpus_key"])

    ability_category_a_keys: set[str] = set()
    with open(args.ability_category_rows, "r", encoding="utf-8") as fh:
        for line in fh:
            r = json.loads(line)
            if r["disposition"] == "A":
                ability_category_a_keys.add(r.get("key") or r["identity"])
    tracked_keys["ability_category_A"] = ability_category_a_keys

    # KEY:-field-only set, retained separately for the strict cross-kind
    # duplicate test (B-duplicate) -- this one keeps the shared-name-hazard
    # discipline, never falling back to bare identity.
    tracked_keys_strict: dict[str, set[str]] = collections.defaultdict(set)
    for bd in scope.in_scope:
        book_dir = os.path.join(pathfinder_root, bd.rel_path)
        for dirpath, _dirnames, filenames in os.walk(book_dir):
            for fn in sorted(filenames):
                if not fn.lower().endswith(".lst"):
                    continue
                full = os.path.join(dirpath, fn)
                bucket, key = ci._classify_kind_by_filename(fn, bd.book_id)
                if bucket == "row_dependent":
                    for identity, raw in ci._parse_lst_rows(full):
                        cat = ci._row_category_tag(raw)
                        if cat and cat.upper() == "FEAT":
                            ident_upper = identity.upper()
                            if ident_upper.endswith((".FORGET", ".MOD")):
                                continue
                            kf = _key_field(raw)
                            if kf:
                                tracked_keys_strict["feat"].add(kf)
                    continue
                if bucket != "kind":
                    continue
                for identity, raw in ci._parse_lst_rows(full):
                    ident_upper = identity.upper()
                    if ident_upper.endswith((".FORGET", ".MOD")):
                        continue
                    kf = _key_field(raw)
                    if kf:
                        tracked_keys_strict[key].add(kf)

    all_resolvable_keys: set[str] = set()
    for keyset in tracked_keys.values():
        all_resolvable_keys |= keyset

    # ------------------------------------------------------------------
    # Pass 2: the 2,614 CATEGORY:Internal rows in _abilities_class.lst
    # files, walked independently of census_independent.py's own bucket
    # label (re-derives the file-kind classification, does not trust the
    # `row_dependent_class_feature` -> `ability_category:Internal` label
    # already applied there).
    # ------------------------------------------------------------------
    raw_rows = []
    for bd in scope.in_scope:
        book_dir = os.path.join(pathfinder_root, bd.rel_path)
        for dirpath, _dirnames, filenames in os.walk(book_dir):
            for fn in sorted(filenames):
                if not fn.lower().endswith(".lst"):
                    continue
                full = os.path.join(dirpath, fn)
                bucket, _key = ci._classify_kind_by_filename(fn, bd.book_id)
                if bucket != "row_dependent_class_feature":
                    continue
                for identity, raw_line in ci._parse_lst_rows(full):
                    cat = ci._row_category_tag(raw_line)
                    if not (cat and cat.upper() == "INTERNAL"):
                        continue
                    ident_upper = identity.upper()
                    if ident_upper.endswith((".FORGET", ".MOD")):
                        continue
                    raw_rows.append((bd.book_id, full, identity, raw_line))

    # Pass 2a: content/formula/gateway signals computed once per row, plus
    # the within-population resolution set -- a gateway row inside this same
    # 2,614-row population may target a SIBLING row in the same population
    # that itself carries independent content (e.g. "Brawler Unarmed Damage
    # LVL 1" gateways to 9 sibling "...(Fine)"/"...(Diminutive)"/... rows,
    # each its own BONUS:VAR-bearing row in this same file). Resolving only
    # against the *external* tracked-kind population would wrongly mark
    # these as unresolved.
    prelim = []
    within_population_resolved: set[str] = set()
    for book_id, full, identity, raw_line in raw_rows:
        has_content_base = bool(CONTENT_RE_BASE.search(raw_line))
        has_content_ext = bool(CONTENT_RE_EXT.search(raw_line))
        has_content_full = bool(CONTENT_RE_FULL.search(raw_line))
        has_formula = bool(FORMULA_RE.search(raw_line))
        has_gateway = bool(GATEWAY_RE.search(raw_line))
        gw_targets = [t.strip() for t in GATEWAY_TARGET_RE.findall(raw_line)]
        key = _key_field(raw_line)
        prelim.append((book_id, full, identity, raw_line, has_content_base,
                        has_content_ext, has_content_full, has_formula, has_gateway, gw_targets, key))
        if has_content_full:
            within_population_resolved.add(key or identity)

    resolution_universe = all_resolvable_keys | within_population_resolved

    rows = []
    for (book_id, full, identity, raw_line, has_content_base, has_content_ext,
         has_content_full, has_formula, has_gateway, gw_targets, key) in prelim:
        gw_resolved = (
            any(t in resolution_universe for t in gw_targets)
            if gw_targets else None
        )

        collided_kind = None
        if key:
            for kind, keyset in tracked_keys_strict.items():
                if key in keyset:
                    collided_kind = kind
                    break

        # Disposition priority, identical order to the sibling
        # ability_category classifier: duplicate > content > gateway > picklist.
        # Uses the FULL content test as the disposition-of-record (base/ext
        # are reported alongside for reconciliation -- see summary). An
        # UNRESOLVED gateway is not proven (B) -- decisions.md §12b puts the
        # burden of proof on (B); a row this script cannot prove is a facet
        # of an already-counted unit stays disposed (A) rather than being
        # silently excluded (see "A (unresolved-gateway)" in the summary).
        if collided_kind:
            disposition = "B-duplicate"
        elif has_content_full:
            disposition = "A"
        elif has_gateway and gw_resolved:
            disposition = "B-gateway-resolved"
        elif has_gateway:
            disposition = "A-unresolved-gateway"
        else:
            disposition = "B-picklist"

        which_tokens = sorted(set(re.findall(
            r"\b(SPELLKNOWN[A-Z]*|BONUS[A-Z]*|ABILITY|DEFINE|TEMPBONUS|AUTO|DESC|ASPECT|CSKILL|MOVE|TEMPLATE|SPROP|QUALITY|SR|DR|SAB|VISION|"
            r"CHOOSE|NATURALATTACKS|COMPANIONLIST|ADD|FOLLOWERS|UDAM|UMULT|SELECT|COST|MOVECLONE|SPELLS|SERVESAS|DEFINESTAT|UNENCUMBEREDMOVE|BENEFIT|SPELLLEVEL|CMB)(?=:)",
            raw_line,
        )))

        rows.append(
            {
                "book_id": book_id,
                "file": os.path.relpath(full, pathfinder_root),
                "identity": identity,
                "key": key,
                "has_content_base": has_content_base,
                "has_content_ext": has_content_ext,
                "has_content_full": has_content_full,
                "has_formula": has_formula,
                "has_gateway": has_gateway,
                "gateway_targets": gw_targets,
                "gateway_resolved": gw_resolved,
                "collided_kind": collided_kind,
                "tokens": which_tokens,
                "disposition": disposition,
                "line": raw_line,
            }
        )

    with open(args.output_jsonl, "w", encoding="utf-8") as fh:
        for r in rows:
            fh.write(json.dumps(r) + "\n")

    # Self-check: 2,614 is the population named in decisions.md §14c item 4 /
    # census_independent.py's own comment at the `row_dependent_class_feature`
    # branch. Independently re-derived here by walking the corpus with the
    # file-kind rule alone (bucket == row_dependent_class_feature, cat ==
    # INTERNAL), not by trusting the diff.json label already applied.
    expected = 2614
    mismatch = len(rows) != expected

    disp_counts = collections.Counter(r["disposition"] for r in rows)
    token_counts = collections.Counter()
    for r in rows:
        for t in r["tokens"]:
            token_counts[t] += 1

    n_base_content = sum(1 for r in rows if r["has_content_base"])
    n_ext_content = sum(1 for r in rows if r["has_content_ext"])
    n_full_content = sum(1 for r in rows if r["has_content_full"])
    n_formula = sum(1 for r in rows if r["has_formula"])
    n_gateway = sum(1 for r in rows if r["has_gateway"])
    n_gateway_resolved = sum(1 for r in rows if r["has_gateway"] and r["gateway_resolved"])
    n_gateway_unresolved = sum(1 for r in rows if r["has_gateway"] and r["gateway_resolved"] is False)
    n_neither = sum(
        1 for r in rows if not r["has_content_full"] and not r["has_gateway"]
    )
    n_a_total = disp_counts.get("A", 0) + disp_counts.get("A-unresolved-gateway", 0)
    n_b_total = (
        disp_counts.get("B-gateway-resolved", 0)
        + disp_counts.get("B-picklist", 0)
        + disp_counts.get("B-duplicate", 0)
    )

    with open(args.output_summary_md, "w", encoding="utf-8") as fh:
        fh.write("# Card 15 -- CATEGORY:Internal (`_abilities_class.lst`) per-row disposition\n\n")
        fh.write(f"Rows found (independent re-walk, bucket=`row_dependent_class_feature`, "
                 f"cat=INTERNAL): **{len(rows)}** (expected {expected} -- "
                 f"{'MATCH' if not mismatch else 'MISMATCH'})\n\n")
        fh.write(f"**Disposition of record: {n_a_total} (A) / {n_b_total} (B)** "
                 f"({100*n_a_total/len(rows):.1f}% / {100*n_b_total/len(rows):.1f}%).\n\n")
        fh.write("## Disposition (full content test -- see module docstring for the field list; "
                  "disposition-of-record)\n\n")
        fh.write("| disposition | count | meaning |\n|---|---:|---|\n")
        fh.write(f"| A | {disp_counts.get('A', 0)} | proven independent mechanical/narrative content |\n")
        fh.write(f"| A-unresolved-gateway | {disp_counts.get('A-unresolved-gateway', 0)} | gateway token whose target this script could not resolve -- **not proven (B)**, so stays (A) per decisions.md §12b's burden of proof |\n")
        fh.write(f"| B-gateway-resolved | {disp_counts.get('B-gateway-resolved', 0)} | proven facet -- gateway target resolves to an already-counted real object |\n")
        fh.write(f"| B-picklist | {disp_counts.get('B-picklist', 0)} | proven inert -- zero content field, zero gateway token |\n")
        fh.write(f"| B-duplicate | {disp_counts.get('B-duplicate', 0)} | exact KEY: match on a tracked kind elsewhere |\n")
        fh.write(f"| **TOTAL** | **{sum(disp_counts.values())}** | |\n\n")

        fh.write("## Content-test comparison (same 2,614 rows, four definitions)\n\n")
        fh.write("| test | rows matching | rows NOT matching |\n|---|---:|---:|\n")
        fh.write(f"| base (ability_category classifier's own list) | {n_base_content} | {len(rows) - n_base_content} |\n")
        fh.write(f"| extended (+ SPELLKNOWN*/TEMPBONUS -- verifier's 6 token families) | {n_ext_content} | {len(rows) - n_ext_content} |\n")
        fh.write(f"| full (+ CHOOSE/NATURALATTACKS/COMPANIONLIST/ADD/FOLLOWERS/UDAM/UMULT/SELECT/COST/MOVECLONE/SPELLS/SERVESAS/DEFINESTAT/UNENCUMBEREDMOVE/BENEFIT -- this script's disposition-of-record) | {n_full_content} | {len(rows) - n_full_content} |\n")
        fh.write(f"| formula-only (DEFINE/BONUS*, class_feature memo's/shape_ledger's test) | {n_formula} | {len(rows) - n_formula} |\n\n")

        fh.write("## Gateway resolution (own KEY:-or-identity join, scoped per target kind; "
                  "resolution universe = tracked kinds + inventory's tracked class_feature units + "
                  "ability_category lane's own disposition-A rows + sibling A-disposition rows within "
                  "this same 2,614-row population)\n\n")
        fh.write(f"- rows with an `ABILITY:...\\|AUTOMATIC\\|<target>` token: {n_gateway}\n")
        fh.write(f"- of those, target resolves to an already-counted unit: {n_gateway_resolved}\n")
        fh.write(f"- of those, target does NOT resolve (stays disposed A -- not proven B): {n_gateway_unresolved}\n\n")

        fh.write(f"## Rows with neither full content nor a gateway token: {n_neither}\n\n")

        fh.write("## Per-token presence (not mutually exclusive; a row may carry several)\n\n")
        fh.write("| token | rows carrying it |\n|---|---:|\n")
        for tok, cnt in token_counts.most_common():
            fh.write(f"| {tok} | {cnt} |\n")

        fh.write("\n## Per-book distribution of disposition A (A + A-unresolved-gateway)\n\n")
        by_book_a = collections.Counter(
            r["book_id"] for r in rows if r["disposition"] in ("A", "A-unresolved-gateway")
        )
        fh.write("| book | A rows |\n|---|---:|\n")
        for book, cnt in by_book_a.most_common():
            fh.write(f"| {book} | {cnt} |\n")

    print(f"rows: {len(rows)} (expected {expected})")
    print(f"self-check: {'MATCH' if not mismatch else 'MISMATCH'}")
    print(f"disposition: {dict(disp_counts)}")
    print(f"A_total={n_a_total} B_total={n_b_total}")
    print(f"content(base)={n_base_content} content(ext)={n_ext_content} content(full)={n_full_content} "
          f"formula={n_formula} gateway={n_gateway} gateway_resolved={n_gateway_resolved} "
          f"gateway_unresolved={n_gateway_unresolved} neither={n_neither}")
    return 1 if mismatch else 0


if __name__ == "__main__":
    raise SystemExit(main())
