#!/usr/bin/env python3
"""scripts/observer/provenance.py -- Decision 14 (CONFIRMED 2026-08-17,
operator ruling), `SD-31-corpus-closure-grind/decisions.md`: the provenance
schema, a NEW AXIS distinct from doneness.

THE AXIS. The board already has a *doneness* axis (`wiring_class` x `status`
-> `not-started`/`held`/`done`, `§7`). Provenance answers a DIFFERENT
question: does this `(object, book)` pair belong in the mandate denominator
at all, and which book owns it? **Provenance decides denominator MEMBERSHIP.
Doneness measures progress WITHIN it. Neither may be derived from the
other**, and a provenance change must move ZERO doneness fields (invariant 5
below) -- conflating the two would let a provenance edit silently move the
`done` percentage, exactly the Decision 1(a) anti-gaming violation this
package forbids.

THE NINE STATUSES, one per `(object, book)` pair, no default:

  origin              the original publish; authoritative until corrected
  duplicate           an identical later reprint (`§13` branch 1) -- origin
                       is untouched and stays `origin`
  superseded          origin FLIPPED here because a later book corrected
                       the values (`§13` branch 3) -- this pair holds the
                       outdated values and is no longer authoritative
  errata-source       the later book whose corrected values now win;
                       flips the origin pair to `superseded`. At most ONE
                       per object (successive errata: only the LATEST is
                       `errata-source`, every earlier pair -- including the
                       original publish -- is `superseded`)
  variant             a distinct derived object sharing a lineage --
                       sub-race, Unchained, Mythic (`§13` branch 2). Its own
                       record; it is an `origin` in its own right and this
                       status records only the lineage
  descoped-licensing   cannot be shipped for licensing / declared-PI reasons
  descoped-structural  the `§3` Structural Exclusion Register; finishing is
                       genuinely impossible -- OPERATOR-SIGNED, never a
                       cycle's own call
  packaging-artifact   the pair names a PCGen packaging directory, not a
                       real book (`§9`, `core_essentials`) -- MUST trend to
                       zero as re-attribution completes
  out-of-roster        resolves to a real book outside the 37-book mandate
                       roster (`§9`'s census: Ironfang Invasion, Blood of
                       the Moon, Universal Rules) -- a scope question, not a
                       deletion, pending operator

A TENTH, NON-CANONICAL bookkeeping value, `UNCLASSIFIED`, is provided
alongside the nine for exactly one purpose: recording "not yet decided"
honestly. `§13`'s AMENDMENT records that the operator is waiting on a
worked per-race evidence table before ruling branch 1/2/3 for any object
printed in more than one book, and race attribution stays FROZEN until then.
A confidently wrong provenance label is precisely the defect `§9` was
created to fix ("i dont want duplicates falsely adding to the denominator")
-- so this module classifies only what is unambiguous by construction
(`classify_unambiguous` below) and leaves everything else `UNCLASSIFIED`
rather than guessing. `UNCLASSIFIED` never counts toward the denominator
(`compute_denominator` excludes it, same as every non-authoritative,
non-variant status) and a totality check against the full universe still
passes with it present -- see `check_totality`'s own docstring for exactly
what totality does and does not claim.

**THIS CYCLE DOES NOT APPLY THE SCHEMA TO THE MANDATE DENOMINATOR.** The
Supersession Register rebuild (a separate lane) has not landed, and race
attribution is frozen per `§13`'s amendment. This module ships the schema,
its gates (each proven able to fail -- see `scripts/tests/
test_provenance.py`), and a classification pass restricted to what needs no
race-evidence ruling at all. No call site in `pf1e_dashboard_producer.py`
reads this module's `compute_denominator()` this cycle; wiring it into the
live mandate figure is future work for whichever lane consumes the register
rebuild.
"""
from __future__ import annotations

import dataclasses

# --- The nine canonical statuses, plus the one bookkeeping sentinel -------

ORIGIN = "origin"
DUPLICATE = "duplicate"
SUPERSEDED = "superseded"
ERRATA_SOURCE = "errata-source"
VARIANT = "variant"
DESCOPED_LICENSING = "descoped-licensing"
DESCOPED_STRUCTURAL = "descoped-structural"
PACKAGING_ARTIFACT = "packaging-artifact"
OUT_OF_ROSTER = "out-of-roster"

# Bookkeeping only -- NOT one of the nine canonical statuses. See module
# docstring. Never counts toward the denominator; never satisfies invariant
# 2's "exactly one authoritative pair."
UNCLASSIFIED = "unclassified"

CANONICAL_STATUSES = (
    ORIGIN, DUPLICATE, SUPERSEDED, ERRATA_SOURCE, VARIANT,
    DESCOPED_LICENSING, DESCOPED_STRUCTURAL, PACKAGING_ARTIFACT, OUT_OF_ROSTER,
)
ALL_STATUSES = CANONICAL_STATUSES + (UNCLASSIFIED,)

# Statuses that count toward `denominator = authoritative + variant`
# (invariant 3). `origin` and `errata-source` are the two shapes
# "authoritative" can take (Decision 14's forced correction to its own
# original invariants 2/3 -- an object WITH errata has NO `origin` pair at
# all, it flipped, so `origin` alone is not the right test).
AUTHORITATIVE_STATUSES = (ORIGIN, ERRATA_SOURCE)
DENOMINATOR_STATUSES = AUTHORITATIVE_STATUSES + (VARIANT,)


@dataclasses.dataclass(frozen=True)
class ProvenanceRecord:
    """One `(object, book)` pair's provenance classification.

    `object_id` identifies the OBJECT across books (Decision 14 is explicit
    the pair is `(object, book)`, not `(book,)` alone) -- callers own how
    they derive it (`(kind, corpus_key)` is `§10`'s own guard: match on
    that, never on a bare shared NAME). `basis` records WHY this pair got
    this status, human-readable, so a classification is auditable rather
    than asserted -- every populated record in this module's own output
    carries one; a gate does not require it, but `check_totality`'s
    companion reporting surfaces records with an empty basis as worth a
    second look.
    """
    object_id: str
    book: str
    status: str
    basis: str = ""

    def __post_init__(self):
        if self.status not in ALL_STATUSES:
            raise ValueError(
                f"unknown provenance status {self.status!r} for "
                f"({self.object_id!r}, {self.book!r}) -- must be one of {ALL_STATUSES}"
            )


@dataclasses.dataclass(frozen=True)
class Violation:
    kind: str
    detail: str


# --- Invariant 1: totality --------------------------------------------------

def check_totality(records: list[ProvenanceRecord], universe: set[tuple[str, str]]) -> list[Violation]:
    """Every `(object, book)` pair in `universe` carries EXACTLY ONE
    status, no default. `UNCLASSIFIED` satisfies this (it is a real,
    explicit value, not an absence) -- what totality forbids is a pair
    with ZERO records (never classified at all, not even as unclassified)
    or MORE THAN ONE record (two different statuses asserted for the same
    pair, which is a genuine conflict, not "the comparison never ran" --
    that shape belongs to invariant 2, about AUTHORITATIVE pairs
    specifically, not every pair).

    Returns one violation per pair that is missing or duplicated. An empty
    list is the passing case."""
    by_pair: dict[tuple[str, str], list[ProvenanceRecord]] = {}
    for r in records:
        by_pair.setdefault((r.object_id, r.book), []).append(r)

    violations: list[Violation] = []
    for pair in sorted(universe):
        rows = by_pair.get(pair, [])
        if not rows:
            violations.append(Violation("MISSING", f"{pair} carries no provenance record at all"))
        elif len(rows) > 1:
            statuses = sorted({r.status for r in rows})
            violations.append(Violation("DUPLICATE", f"{pair} carries {len(rows)} records ({statuses})"))
    # A record for a pair OUTSIDE the stated universe is also a totality
    # defect in the other direction -- it means the universe itself is
    # stale/wrong, and a caller silently trusting `records` over `universe`
    # would under-report `MISSING` above.
    for pair in sorted({(r.object_id, r.book) for r in records} - universe):
        violations.append(Violation("OUT_OF_UNIVERSE", f"{pair} has a provenance record but is not in the stated universe"))
    return violations


# --- Invariant 2: exactly one authoritative pair per object ---------------

def check_exactly_one_authoritative(records: list[ProvenanceRecord]) -> list[Violation]:
    """Exactly one AUTHORITATIVE pair (`origin`, or its single
    `errata-source` if one exists) per OBJECT -- across every book that
    object appears in, not per book. Zero authoritative pairs means the
    object is unowned (a genuine gap); two means the `§13` comparison was
    never applied (an object cannot simultaneously have an untouched
    original publish AND a correcting errata both still claiming
    authority -- the whole point of the origin-flip mechanic is that only
    one wins). `UNCLASSIFIED` pairs are excluded from this count entirely
    (an object with only unclassified appearances has not been ruled on
    yet, which is a different, honest state from "zero authoritative" --
    callers that need to distinguish the two read `check_totality`'s
    output alongside this one)."""
    by_object: dict[str, list[ProvenanceRecord]] = {}
    for r in records:
        by_object.setdefault(r.object_id, []).append(r)

    violations: list[Violation] = []
    for obj, rows in sorted(by_object.items()):
        if all(r.status == UNCLASSIFIED for r in rows):
            continue
        authoritative = [r for r in rows if r.status in AUTHORITATIVE_STATUSES]
        if len(authoritative) == 0:
            violations.append(Violation("UNOWNED", f"object {obj!r} has zero authoritative (origin/errata-source) pairs"))
        elif len(authoritative) > 1:
            books = sorted(r.book for r in authoritative)
            violations.append(Violation(
                "AMBIGUOUS_AUTHORITY",
                f"object {obj!r} has {len(authoritative)} authoritative pairs across books {books} "
                f"-- the §13 comparison was never applied",
            ))
    return violations


# --- Invariant 3: denominator = authoritative + variant, derived ----------

def compute_denominator(records: list[ProvenanceRecord]) -> dict:
    """`denominator = authoritative + variant` (`origin + errata-source +
    variant`), NEVER hand-maintained -- this is the only function in this
    module that is allowed to produce that number, and it is a pure fold
    over `records`. Returns the total plus the count per status (invariant
    6's own reporting requirement), so a caller reporting a denominator
    change can cite both numbers from one call."""
    by_status: dict[str, int] = {s: 0 for s in ALL_STATUSES}
    for r in records:
        by_status[r.status] = by_status.get(r.status, 0) + 1
    denominator = sum(by_status.get(s, 0) for s in DENOMINATOR_STATUSES)
    return {"denominator": denominator, "by_status": by_status}


# --- Invariant 4: packaging-artifact -> zero; descoped-structural signed --

def check_packaging_artifact_trending_to_zero(records: list[ProvenanceRecord]) -> list[Violation]:
    """`packaging-artifact` is a TRANSITIONAL state, not a resting place
    (`§9`) -- it must trend to zero as re-attribution completes and is a
    hard failure once that re-attribution is declared done. This function
    reports every remaining `packaging-artifact` pair; the CALLER decides
    whether "re-attribution complete" has been declared yet (this module
    has no opinion on that separately-tracked milestone) -- see
    `scripts/tests/test_provenance.py` for how a cycle proves this can
    fail: seed one `packaging-artifact` record and assert it is reported."""
    return [
        Violation("PACKAGING_ARTIFACT_REMAINING", f"({r.object_id!r}, {r.book!r}) still classified packaging-artifact")
        for r in records if r.status == PACKAGING_ARTIFACT
    ]


def check_descoped_structural_signed(records: list[ProvenanceRecord], signed_pairs: set[tuple[str, str]]) -> list[Violation]:
    """Every `descoped-structural` pair must have a matching operator
    signature in `signed_pairs` (the `§3` Structural Exclusion Register --
    a cycle may PROPOSE a pair for this status but only the operator's
    signed register entry actually grants it, per `§3`/`AT-31-100`, "a
    cycle may propose; only the operator grants"). `descoped-licensing`
    needs no signature (it needs declared-PI evidence instead, which this
    gate does not check -- that evidence lives with the redaction path,
    `decisions.md §12`)."""
    return [
        Violation("UNSIGNED_STRUCTURAL_EXCLUSION", f"({r.object_id!r}, {r.book!r}) is descoped-structural with no operator signature in the register")
        for r in records
        if r.status == DESCOPED_STRUCTURAL and (r.object_id, r.book) not in signed_pairs
    ]


# --- Invariant 5: a provenance change moves ZERO doneness fields ----------

def assert_provenance_change_does_not_move_doneness(
    doneness_before: dict[str, str], doneness_after: dict[str, str]
) -> list[Violation]:
    """`doneness_before`/`doneness_after` map a unit id to its doneness
    verdict (`not-started`/`held`/`done`/etc, whatever
    `pf1e_dashboard_producer.doneness_verdict()` returns) taken immediately
    before and after a provenance-ONLY commit. A provenance edit may add,
    remove, or reclassify `(object, book)` pairs' MEMBERSHIP -- it must
    NEVER change what a unit that remains present already measures as.
    Returns one violation per unit whose doneness verdict moved; a unit
    that is present in one snapshot and absent in the other (its provenance
    changed membership, the expected effect of this kind of commit) is not
    itself a violation -- only a verdict that DIFFERS for a unit present in
    BOTH snapshots is."""
    violations: list[Violation] = []
    for unit_id, before in doneness_before.items():
        if unit_id not in doneness_after:
            continue
        after = doneness_after[unit_id]
        if before != after:
            violations.append(Violation(
                "DONENESS_MOVED_BY_PROVENANCE_CHANGE",
                f"unit {unit_id!r} doneness moved {before!r} -> {after!r} across a provenance-only change",
            ))
    return violations


# --- Invariant 6: denominator change reported as its own number ----------

def report_denominator_change(before: dict, after: dict) -> dict:
    """`before`/`after` are `compute_denominator()` results. Returns a
    structured delta -- the total change AND the per-status change --
    "never incidental" (`§14` invariant 7/`§5`): a caller must not fold
    this into a generic diff and must cite these numbers explicitly."""
    before_by_status = before.get("by_status", {})
    after_by_status = after.get("by_status", {})
    per_status_delta = {
        s: after_by_status.get(s, 0) - before_by_status.get(s, 0)
        for s in ALL_STATUSES
        if after_by_status.get(s, 0) != before_by_status.get(s, 0)
    }
    return {
        "denominator_before": before.get("denominator", 0),
        "denominator_after": after.get("denominator", 0),
        "denominator_delta": after.get("denominator", 0) - before.get("denominator", 0),
        "per_status_delta": per_status_delta,
    }


# --- Population: classify only what needs no race-evidence ruling --------

def classify_unambiguous(object_book_pairs: list[tuple[str, str, str]], packaging_artifact_books: set[str]) -> list[ProvenanceRecord]:
    """`object_book_pairs` is `(object_id, book, kind)` triples (kind kept
    for a legible `basis` string only; it plays no role in the
    classification logic itself). Returns one `ProvenanceRecord` per
    distinct `(object_id, book)` pair, using ONLY rules that need no
    content comparison and no race-evidence ruling:

    - `packaging_artifact_books` (Decision 9, `§9`'s already-settled
      finding -- `core_essentials` is not a real book) -> every pair in
      one of these books is `packaging-artifact`, unconditionally. This is
      the one already-decided case Decision 9 hands this module directly.
    - An object appearing in EXACTLY ONE book (across the whole input) has
      no competing printing to compare against at all -- `§13`'s three-
      branch test is a comparison BETWEEN printings, and with only one
      printing there is nothing to compare, so it is trivially `origin`.
    - An object appearing in MORE THAN ONE book needs the `§13` branch
      1/2/3 content comparison this module deliberately does not attempt
      (that ruling is `§13`'s AMENDMENT's own owed work, the per-race
      worked-example evidence table, still pending the operator) --
      classified `UNCLASSIFIED` for every one of its pairs, honestly, per
      this module's own docstring."""
    by_object: dict[str, list[tuple[str, str]]] = {}
    for obj, book, kind in object_book_pairs:
        by_object.setdefault(obj, []).append((book, kind))

    records: list[ProvenanceRecord] = []
    for obj, book_kinds in sorted(by_object.items()):
        books = [bk for bk, _kind in book_kinds]
        for book, kind in book_kinds:
            if book in packaging_artifact_books:
                records.append(ProvenanceRecord(
                    object_id=obj, book=book, status=PACKAGING_ARTIFACT,
                    basis=f"{book!r} is a PCGen packaging directory, not a real book (§9)",
                ))
            elif len(set(books)) == 1:
                records.append(ProvenanceRecord(
                    object_id=obj, book=book, status=ORIGIN,
                    basis=f"only printing of this {kind} anywhere in the input; no comparison needed",
                ))
            else:
                records.append(ProvenanceRecord(
                    object_id=obj, book=book, status=UNCLASSIFIED,
                    basis=(
                        f"printed in {sorted(set(books))}; §13 branch 1/2/3 content "
                        f"comparison not yet ruled on (race evidence table pending)"
                    ),
                ))
    return records
