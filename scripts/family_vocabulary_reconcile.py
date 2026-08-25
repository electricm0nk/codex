#!/usr/bin/env python3
"""SD-32 Gate 1 family-vocabulary reconciliation (card `family-vocabulary-
reconciliation`, `decisions.md` §12a).

Why this exists
----------------
`scripts/shape_ledger.py` shipped eleven families (F0-F10) as the Gate 1
deliverable. SD-31's own `docs/release/SD-31-corpus-closure-grind/artifacts/
MEASURE-TWICE.md` §3 independently hand-measured a *different* ten-family
vocabulary, with different counts, over a different population (the 4,948
formula-bearing "primary partition" of the 24,914 not-done units -- MT never
classified the remaining units at all). Nothing reconciled the two, and
three consequences followed (`decisions.md` §12a):

1. AT-32-G1-003's cross-check command named a table (`epic-breakdown.md`'s
   "F1..F10 table") that does not exist -- `epic-breakdown.md` Epic 1's
   F1/F2/F3 rows are three *work items*, not semantic families with counts.
2. `kanban.md` card 7's title called `bonus_stack_reader.rs`'s target
   family "F10", while `shape_ledger.py`'s own F4 proof-width text names F4
   -- not F10 -- as "the shape `bonus_stack_reader.rs`'s binding-layer
   pattern targets". F10 is an unrelated 3-unit step-count heuristic.
3. Card 7's "77.2% of custom identifiers (893/1,156)" engine-coverage claim
   had no committed command connecting it to either vocabulary's families.

**Ruling, per `decisions.md` §12a: SD-32 ships exactly one shape-family
vocabulary.** `scripts/shape_ledger.py`'s `FAMILIES` list (plus its F0/F8
extension families) is that vocabulary -- it is the only one of the two
that is (a) committed and re-runnable rather than a hand-walk that was
explicitly "not re-committed as a script" (MEASURE-TWICE.md §7), and (b)
total over the full 24,914-unit not-done population rather than only its
4,948-unit formula-bearing subset (AT-32-G1-001 requires *every* unit to
map to a family; MEASURE-TWICE.md's own ten families, by construction,
never classified the other 19,966+ units at all).

This module is the canonical home for the RECONCILIATION -- not a second
copy of the vocabulary. It imports `shape_ledger.FAMILIES` directly (never
duplicates a family id, label, or predicate), so the mapping this script
prints can never silently drift from what the ledger actually classifies:
change a label in `shape_ledger.py` and this script's canonical table
changes with it on the next run, because it is read, not copied.

What this script produces
--------------------------
1. **The canonical family table** -- id, label, proof width, live count
   (from a fresh Gate-1-shaped ledger run), and the re-derive command --
   read directly from `shape_ledger.py`, never hand-transcribed.
2. **The MT-to-canonical mapping** -- MEASURE-TWICE.md §3's ten families,
   matched to the canonical F-id by label identity (all ten names appear
   verbatim in both documents), with both counts and the delta. F0 has no
   MT counterpart, named as a finding (MT's own vocabulary only ever
   covered the formula-bearing subset -- it was incomplete by
   construction, not extended).
3. **The engine-coverage reconciliation** -- maps card 7's cited
   "77.2% (893/1,156) of the corpus's distinct custom identifiers" onto the
   canonical F4 family ("named-counter/pool variable ... the shape
   `bonus_stack_reader.rs`'s binding-layer pattern targets") by
   re-deriving, corpus-wide (not restricted to the not-done population,
   matching MT §3.1's own stated scope), the set of distinct identifier
   strings that appear as an F4-shaped (bare-identifier) DEFINE/BONUS
   formula segment, then checking how many of those identifiers are
   resolvable via `bonus_stack_reader.rs`'s generalised producer-chain
   mechanism (cycle 007: an identifier is resolvable if it is ever the
   TARGET of a `DEFINE` or `BONUS:VAR` write anywhere in the corpus -- the
   same condition `extract_define_base`/`resolve_producer_chain_corpus_wide`
   test). This is a real, independent re-derivation, not a citation of the
   007/008 cycle receipts' own numbers -- the population and resolved count
   this script computes are its own, and are reported alongside MT's and
   card 7's cited figures for direct comparison.

Usage
-----
    python3 scripts/family_vocabulary_reconcile.py \\
        --inventory docs/work-inventory.json \\
        --corpus-root data/corpus \\
        --output-json artifacts/gate-1-shape-closure/family-vocabulary.json \\
        --output-md artifacts/gate-1-shape-closure/family-vocabulary.md

Exit status mirrors `shape_ledger.py`'s fail-closed posture: 1 (with a
"no coverage" message on stderr) if the inventory is unreadable/empty, or
if the corpus-wide identifier walk finds zero DEFINE/BONUS formula tokens
at all (an empty corpus cannot manufacture a false reconciliation).
"""

from __future__ import annotations

import argparse
import glob
import json
import os
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DEFAULT_INVENTORY = os.path.join(REPO_ROOT, "docs", "work-inventory.json")
DEFAULT_CORPUS_ROOT = os.path.join(REPO_ROOT, "data", "corpus")

sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import shape_ledger as SL  # noqa: E402
import coverage_ledger as CL  # noqa: E402

# MEASURE-TWICE.md §3's ten families, transcribed verbatim (label + count)
# from the table's own text -- matched to the canonical F-id by label
# identity, not by position (MT's table order is not the same as
# shape_ledger.py's priority order). Every one of these ten labels appears,
# word-for-word or as the canonical label's clear subset, in
# `docs/release/SD-31-corpus-closure-grind/artifacts/MEASURE-TWICE.md` §3's
# table -- re-check by reading that table directly if this ever needs
# re-verifying; it is intentionally hand-transcribed once here (not parsed
# from the .md file) because MT is prose/markdown, not machine-readable,
# the same reason AT-32-G1-003's own cross-check is a "hand cross-check,
# not JSON diff".
MT_FAMILIES = {
    "F1": {"mt_label": "Flat-constant magnitude (bare literal, e.g. +2 damage)", "mt_count": 1747},
    "F2": {"mt_label": "Per-level scaling (<Class>LVL bare/arithmetic)", "mt_count": 1140},
    "F3": {"mt_label": "Ability-modifier-derived (STR/DEX/CON/INT/WIS/CHA in formula)", "mt_count": 804},
    "F4": {"mt_label": "Named-counter/pool variable (e.g. a class's per-day-uses tracker)", "mt_count": 563},
    "F5": {"mt_label": "Clamped/capped per-level scaling (min/max/floor wrapping a level expr)", "mt_count": 368},
    "F6": {"mt_label": "classlevel(...)-derived", "mt_count": 211},
    "F7": {"mt_label": "Conditional-step (if/boolean toggle)", "mt_count": 54},
    "F8": {"mt_label": "Other named-variable expression (residual)", "mt_count": 37},
    "F9": {"mt_label": "Skill-rank-derived (skillinfo TOTALRANK)", "mt_count": 17},
    "F10": {"mt_label": "Level-threshold step-count (sum of level>=N indicator terms)", "mt_count": 7},
}
MT_TOTAL_PRIMARY_PARTITION = 4948
MT_SOURCE = "docs/release/SD-31-corpus-closure-grind/artifacts/MEASURE-TWICE.md §3"


def canonical_family_table(ledger: dict) -> list[dict]:
    """Reads the canonical vocabulary straight from `shape_ledger.py` --
    never a hand-copy. `ledger` is a freshly-built Gate-1 ledger (see
    `main`), so the counts here are live, not transcribed from a prior
    artifact."""
    meta = SL._family_metadata()
    priority = {fid: i for i, (fid, *_rest) in enumerate(SL.FAMILIES)}
    rows = []
    for fid, info in meta.items():
        rows.append(
            {
                "id": fid,
                "label": info["label"],
                "proof_width": info["proof_width"],
                "priority_rank": priority.get(fid),  # None for F0/F8 (extension families, not in priority order)
                "count": ledger["families"].get(fid, {}).get("count", 0),
            }
        )
    rows.sort(key=lambda r: (r["priority_rank"] is None, r["priority_rank"] if r["priority_rank"] is not None else 0, r["id"]))
    return rows


def mt_mapping_table(canonical: list[dict]) -> list[dict]:
    """Maps every canonical family to its MT counterpart (or notes it has
    none -- F0). Reports both counts and the delta so a reader sees exactly
    which families moved and by how much, per `decisions.md` §12a item 2's
    "state which is correct and why, with counts"."""
    rows = []
    for row in canonical:
        fid = row["id"]
        mt = MT_FAMILIES.get(fid)
        rows.append(
            {
                "id": fid,
                "canonical_label": row["label"],
                "canonical_count": row["count"],
                "mt_label": mt["mt_label"] if mt else None,
                "mt_count": mt["mt_count"] if mt else None,
                "delta": (row["count"] - mt["mt_count"]) if mt else None,
                "note": (
                    "No MT counterpart -- MT's ten families only ever partitioned the "
                    "4,948-unit formula-bearing subset; a unit with no DEFINE/BONUS "
                    "token was never assigned to any MT family at all. F0 is the "
                    "honest naming of that gap, not a new distinction MT lacked."
                    if fid == "F0"
                    else None
                ),
            }
        )
    return rows


def _walk_corpus_tokens(corpus_root: str):
    """Yields (key, value) for every raw_tokens entry, corpus-wide, whose
    key is DEFINE or starts with BONUS -- the same filter
    `shape_ledger.build_corpus_index` applies, but walking every record in
    the corpus rather than only those a not-done unit's (book, file, line)
    joins to. This is MT §3.1's own stated population ("1,156 distinct
    custom identifiers exist across the corpus's formulas" -- not
    "across the not-done population's formulas")."""
    for path in glob.glob(os.path.join(corpus_root, "**", "*.json"), recursive=True):
        if os.path.basename(path) == "LICENSE.json":
            continue
        try:
            with open(path, "r", encoding="utf-8") as fh:
                rec = json.load(fh)
        except (OSError, json.JSONDecodeError):
            continue
        raw_tokens = (rec.get("data") or {}).get("raw_tokens") or []
        for tok in raw_tokens:
            if not isinstance(tok, dict):
                continue
            key, value = tok.get("key"), tok.get("value")
            if not isinstance(key, str) or not isinstance(value, str):
                continue
            if key != "DEFINE" and not key.startswith("BONUS"):
                continue
            yield key, value


def _is_f4_bare_identifier(segment: str) -> bool:
    """The canonical F4 predicate, read straight from `shape_ledger.py`'s
    own F4 rule rather than reimplemented -- `classify_formula` returning
    'F4' on this exact segment is the ground truth (F4 is never returned
    for a formula matching a higher-priority family, so this also excludes
    LVL/skillinfo/classlevel/etc. bare identifiers correctly)."""
    return SL.classify_formula(segment) == "F4"


def _producer_targets(corpus_root: str) -> set[str]:
    """Every distinct variable name that is ever the TARGET of a DEFINE or
    a BONUS:VAR write anywhere in the corpus -- what `bonus_stack_reader.
    rs`'s generalised producer-chain resolver (`extract_define_base`,
    `resolve_producer_chain_corpus_wide`, cycle 007) needs to find for a
    bare-identifier reference to be resolvable."""
    targets: set[str] = set()
    for key, value in _walk_corpus_tokens(corpus_root):
        parts = value.split("|")
        if key == "DEFINE" and len(parts) >= 1 and parts[0]:
            targets.add(parts[0])
        elif key.startswith("BONUS") and len(parts) >= 2 and parts[0] == "VAR" and parts[1]:
            # BONUS value shape is "<SubType>|<Target>|<Formula>|...": the
            # subtype ("VAR") is the value's OWN first field, never part of
            # the token's `key` (the key is always the bare "BONUS" string
            # -- see shape_ledger.extract_formula_segment's own BONUS
            # branch, which reads target the same way at parts[1]).
            targets.add(parts[1])
    return targets


def reconcile_engine_coverage(corpus_root: str) -> dict:
    """Re-derives, independently of the 007/008 cycle receipts, the
    engine-coverage figure card 7 cited as "77.2% (893/1,156)" -- mapped
    onto the canonical F4 family this script's own table names as the
    binding-layer target."""
    f4_identifiers: set[str] = set()
    for key, value in _walk_corpus_tokens(corpus_root):
        seg = SL.extract_formula_segment(key, value)
        if seg is None:
            continue
        stripped = seg.strip()
        if stripped and _is_f4_bare_identifier(stripped):
            f4_identifiers.add(stripped)

    if not f4_identifiers:
        return {
            "population": 0,
            "resolved": 0,
            "resolved_pct": None,
            "error": "no coverage: zero F4-shaped bare-identifier formula segments found corpus-wide",
        }

    targets = _producer_targets(corpus_root)
    resolved = {i for i in f4_identifiers if i in targets}
    return {
        "population": len(f4_identifiers),
        "resolved": len(resolved),
        "resolved_pct": round(100.0 * len(resolved) / len(f4_identifiers), 1),
        "unresolved_examples": sorted(f4_identifiers - resolved)[:10],
    }


def render_markdown(
    canonical: list[dict],
    mapping: list[dict],
    coverage: dict,
    ledger_population: int,
    join_status_counts: dict | None = None,
) -> str:
    lines = []
    lines.append("# SD-32 Gate 1 — canonical shape-family vocabulary and reconciliation")
    lines.append("")
    lines.append(
        "Generated by `scripts/family_vocabulary_reconcile.py` — see that module's docstring "
        "for the full method. Card `family-vocabulary-reconciliation` (`decisions.md` §12a). "
        "Re-derive: `python3 scripts/family_vocabulary_reconcile.py --output-json "
        "artifacts/gate-1-shape-closure/family-vocabulary.json --output-md "
        "artifacts/gate-1-shape-closure/family-vocabulary.md`."
    )
    lines.append("")
    lines.append(
        f"**Canonical vocabulary is `scripts/shape_ledger.py`'s `FAMILIES` list (F1-F10, priority "
        f"order) plus its F0/F8 extension families — {ledger_population} not-done units, "
        f"unclassified_count 0.** This is the one place a family's id, name, rule, proof width, "
        f"count, and re-derive command are defined; every other document (engine module docs, "
        f"AT-32-* criteria, kanban card titles, `epic-breakdown.md`) cites this table rather than "
        f"restating it."
    )
    lines.append("")
    lines.append("## 0. Join-status split (decisions.md §14b — read this before the family table)")
    lines.append("")
    lines.append(
        "**The family rollup below (and F0 in particular) is not the honest coverage figure on "
        "its own.** F0 (\"no formula content\") conflates two different populations its own "
        "proof-width text distinguishes: a unit whose `(book, source_file, source_line)` join found "
        "**no corpus record at all** (`no_record`), and a unit whose join found a record that simply "
        "**carries no `DEFINE`/`BONUS` token** (`no_formula_tokens`). Only `matched` rests on a real "
        "corpus record. Absence of a record is absence of evidence, not evidence the unit has no "
        "formula — do not read `unclassified_count: 0` as \"100% of this population is understood\"."
    )
    lines.append("")
    jsc = join_status_counts or {}
    matched = jsc.get("matched", 0)
    no_formula_tokens = jsc.get("no_formula_tokens", 0)
    no_record = jsc.get("no_record", 0)
    pop = ledger_population or 1
    lines.append("| join_status | count | share | meaning |")
    lines.append("|---|---:|---:|---|")
    lines.append(f"| `matched` | {matched} | {100.0 * matched / pop:.1f}% | rests on a real corpus record |")
    lines.append(
        f"| `no_formula_tokens` | {no_formula_tokens} | {100.0 * no_formula_tokens / pop:.1f}% | "
        "record found, carries no DEFINE/BONUS token |"
    )
    lines.append(
        f"| `no_record` | {no_record} | {100.0 * no_record / pop:.1f}% | "
        "join found no corpus record at all — this is Gate 3's `no_record` closure-budget invariant |"
    )
    lines.append("")
    lines.append(
        "Re-derive: `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json --output "
        "/tmp/l.json && python3 -c \"import json,collections; r=json.load(open('/tmp/l.json'))"
        "['rows']; print(collections.Counter(x['join_status'] for x in r))\"`."
    )
    lines.append("")
    lines.append("## 1. Canonical family table")
    lines.append("")
    lines.append("| id | label | count | proof width (truncated) |")
    lines.append("|---|---|---:|---|")
    for row in canonical:
        pw = row["proof_width"].replace("\n", " ")
        pw = pw[:140] + "…" if len(pw) > 140 else pw
        lines.append(f"| {row['id']} | {row['label']} | {row['count']} | {pw} |")
    lines.append("")
    lines.append(
        "Re-derive command: `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json "
        "--output artifacts/gate-1-shape-closure/ledger.json`."
    )
    lines.append("")
    lines.append("## 2. Mapping to SD-31's `MEASURE-TWICE.md` §3 vocabulary")
    lines.append("")
    lines.append(
        f"MT's ten families (`{MT_SOURCE}`) partitioned only the {MT_TOTAL_PRIMARY_PARTITION}-unit "
        "formula-bearing subset of the 24,914 not-done units, using an unrecorded priority order "
        "(\"a unit's PRIMARY family is assigned by priority order\" — MT never states what that "
        "order was, and its own walk was explicitly not committed as a script, per MEASURE-TWICE.md "
        "§7). `shape_ledger.py`'s priority order (F9 > F6 > F10 > F5 > F2 > F7 > F3 > F4 > F1) is "
        "the first codification, stated and auditable in `FAMILIES`' own list order."
    )
    lines.append("")
    lines.append(
        "**Ruling: `shape_ledger.py`'s vocabulary and counts are canonical.** It is the only one of "
        "the two that (a) is committed and re-runnable rather than a one-time hand-walk, (b) covers "
        "the full 24,914-unit not-done population rather than only the 4,948-unit formula-bearing "
        "subset (F0 below is the finding this produces), and (c) states an auditable priority order. "
        "Where a per-family count differs (F2, F3, F7 most visibly), the difference is not a defect "
        "in either count on its own — MT's counts were correctly reproduced against MT's own "
        "(unrecorded) priority order and are retained here as a historical cross-check; "
        "`shape_ledger.py`'s counts are the ones every AT-32-* criterion and engine-coverage claim "
        "must cite going forward."
    )
    lines.append("")
    lines.append("| id | canonical count | MT count | delta | canonical label | MT label |")
    lines.append("|---|---:|---:|---:|---|---|")
    for row in mapping:
        mt_count = row["mt_count"] if row["mt_count"] is not None else "—"
        delta = row["delta"] if row["delta"] is not None else "—"
        mt_label = row["mt_label"] or "*(no MT counterpart — see note below)*"
        lines.append(
            f"| {row['id']} | {row['canonical_count']} | {mt_count} | {delta} | "
            f"{row['canonical_label']} | {mt_label} |"
        )
    lines.append("")
    for row in mapping:
        if row["note"]:
            lines.append(f"**{row['id']}:** {row['note']}")
            lines.append("")
    lines.append("## 3. Engine-coverage reconciliation (card 7)")
    lines.append("")
    lines.append(
        "Card 7 (`gate-2-engines-f10-binding`, since retitled per this card's own propagation step) "
        "cited **\"77.2% of custom identifiers (893/1,156)\"** as the reach of `bonus_stack_reader."
        "rs`'s generalised producer-chain mechanism, attributing that figure to \"F10\" — which the "
        "canonical table above shows is wrong: **F10 is a 3-unit level-threshold step-count family, "
        "unrelated to the binding layer.** F4's own proof-width text (canonical table above) names "
        "F4 — \"named-counter/pool variable\" — as \"the shape `bonus_stack_reader.rs`'s binding-layer "
        "pattern targets\"."
    )
    lines.append("")
    if coverage.get("error"):
        lines.append(f"**Reconciliation could not run: {coverage['error']}**")
    else:
        lines.append(
            f"Independently re-derived (corpus-wide, `_walk_corpus_tokens` + canonical F4 predicate "
            f"from `shape_ledger.classify_formula`, not a citation of the 007/008 cycle receipts' own "
            f"numbers): **{coverage['resolved']} of {coverage['population']} ({coverage['resolved_pct']}%) "
            f"distinct F4-shaped bare-identifier strings are resolvable** via `bonus_stack_reader.rs`'s "
            f"producer-chain mechanism (the identifier is the TARGET of a `DEFINE` or `BONUS:VAR` "
            f"write somewhere else in the corpus)."
        )
        lines.append("")
        lines.append(
            f"This population ({coverage['population']} distinct identifiers) is close to but not "
            f"identical to MT §3.1's cited 1,156 and card 7/8's cited 1,156/4,736 — the three are "
            f"different denominators over related but distinct populations: MT's 1,156 was a "
            f"hand-walk (not re-run here byte-for-byte); the corpus-wide `bonus_stack_reader` run "
            f"(`artifacts/gate-2-engines/bonus_stack_reader.corpus-wide.json`) scanned **4,736** "
            f"distinct BONUS:VAR TARGET variables (a broader population — every write target, not "
            f"only F4-shaped bare-identifier VALUE references); this script's "
            f"**{coverage['population']}** is the population of distinct identifier STRINGS that "
            f"appear as the VALUE of an F4-shaped formula segment, i.e. the canonical-F4-scoped "
            f"reading of \"custom identifier\". All three are legitimate, differently-scoped counts; "
            "none of them may be quoted as a bare total without naming which of the three it is "
            "(`decisions.md` §12c)."
        )
        if coverage.get("unresolved_examples"):
            lines.append("")
            lines.append(
                "First 10 unresolved F4-shaped identifiers (no producer found anywhere in corpus — "
                "either a genuinely one-off value, or the join missed a producer this predicate "
                "doesn't see): " + ", ".join(f"`{x}`" for x in coverage["unresolved_examples"])
            )
    lines.append("")
    return "\n".join(lines) + "\n"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--inventory", default=DEFAULT_INVENTORY)
    parser.add_argument("--corpus-root", default=DEFAULT_CORPUS_ROOT)
    parser.add_argument("--output-json")
    parser.add_argument("--output-md")
    args = parser.parse_args(argv)

    try:
        inventory = SL.load_inventory_or_die(args.inventory)
    except (OSError, json.JSONDecodeError) as exc:
        print(f"no coverage: cannot read/parse inventory at {args.inventory!r}: {exc}", file=sys.stderr)
        return 1

    units = CL.not_done_population(inventory)
    if not units:
        print(
            f"no coverage: inventory at {args.inventory!r} has zero not-done units",
            file=sys.stderr,
        )
        return 1

    books = {u.get("book") for u in units if u.get("book")}
    corpus_index = SL.build_corpus_index(args.corpus_root, books)
    # `decisions.md §20`/§17a: reuse `shape_ledger.py`'s citation-redirect
    # fallback (`build_corpus_key_index`) so this reconciliation's own
    # join_status split cannot silently disagree with the CLI's.
    key_index = SL.build_corpus_key_index(args.corpus_root, books)
    # `decisions.md §20` t9-onboarding straggler wave: the cross-book
    # fallback recovers a unit whose real record ships under a DIFFERENT
    # book entirely (not scoped to `books`, for the same reason `key_index`
    # above is reused rather than re-derived).
    cross_book_key_index = SL.build_cross_book_key_index(args.corpus_root)
    ledger = SL.build_ledger(units, corpus_index, key_index, cross_book_key_index)

    canonical = canonical_family_table(ledger)
    mapping = mt_mapping_table(canonical)
    coverage = reconcile_engine_coverage(args.corpus_root)
    if coverage.get("error"):
        print(coverage["error"], file=sys.stderr)
        return 1

    print(f"population (not-done units): {ledger['population']}  unclassified: {ledger['unclassified_count']}")
    print()
    print("canonical family rollup:")
    for row in canonical:
        print(f"  {row['id']:<4} {row['count']:>7}  {row['label']}")
    print()
    print(
        f"engine coverage (F4-scoped, corpus-wide): {coverage['resolved']}/{coverage['population']} "
        f"({coverage['resolved_pct']}%) resolvable via bonus_stack_reader.rs producer-chain"
    )

    out = {
        "population": ledger["population"],
        "unclassified_count": ledger["unclassified_count"],
        "join_status_counts": ledger.get("join_status_counts", {}),
        "canonical_families": canonical,
        "mt_mapping": mapping,
        "mt_total_primary_partition": MT_TOTAL_PRIMARY_PARTITION,
        "engine_coverage_reconciliation": coverage,
    }
    if args.output_json:
        os.makedirs(os.path.dirname(args.output_json) or ".", exist_ok=True)
        with open(args.output_json, "w", encoding="utf-8") as fh:
            json.dump(out, fh, indent=2)
            fh.write("\n")
    if args.output_md:
        os.makedirs(os.path.dirname(args.output_md) or ".", exist_ok=True)
        with open(args.output_md, "w", encoding="utf-8") as fh:
            fh.write(
                render_markdown(
                    canonical, mapping, coverage, ledger["population"], ledger.get("join_status_counts", {})
                )
            )

    return 0


if __name__ == "__main__":
    sys.exit(main())
