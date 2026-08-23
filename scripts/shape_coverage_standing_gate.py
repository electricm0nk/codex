#!/usr/bin/env python3
"""SD-32 Gate 3 closure invariant: the standing gate wired into
`scripts/verify.sh` (AT-32-G3-001/002/003, card `gate-3-closure-invariant`).

Why this exists
----------------
Gate 1's shape ledger (`scripts/shape_ledger.py`) proved, once, that every
not-done unit on 2026-08-22's inventory classifies into one of the ten d20
compute-shape families or an honest extension (F0/F8). A one-time proof is
not a closure invariant: the census, the corpus, and the shape vocabulary
can all move after that proof was written, and nothing re-checked it. This
script is that re-check, made a real `scripts/verify.sh` stage so it runs
on every invocation rather than being a document someone has to remember to
re-open.

It does not duplicate `shape_ledger.py`'s classification rules -- it
**reuses** `build_corpus_index` / `build_ledger` / `classify_unit`
directly, the same "reuse, don't re-derive" discipline `shape_ledger.py`
itself uses for `coverage_ledger.py`'s `not_done_population`. What this
script adds on top:

1. **A live re-run**, not a frozen artifact: every invocation re-derives
   the ledger from the current `docs/work-inventory.json` and the current
   `data/corpus/` tree, so a regression in either is caught the next time
   `scripts/verify.sh` runs, not the next time someone remembers to check.
2. **The sum-the-piles check** (`AGENTS.md` "Concurrency and Measurement" /
   `workflow-instruction.md §9` standing lesson 5): the per-family rollup
   must add back to exactly the population considered. `unclassified_count`
   alone cannot catch a `build_ledger` regression that silently drops rows
   from the `families` rollup while leaving `unclassified_count` at 0 --
   `piles_reconcile` is the independent check for that failure shape.
3. **Fail-closed on an empty predicate** (AT-32-G3-002, Decision 1a): an
   inventory with zero not-done units is not a pass. Mirrors
   `shape_ledger.py`'s and `coverage_ledger.py`'s identical posture.
4. **The corpus SHA citation** (AT-32-G3-003): every report names the
   pinned oracle commit (`scripts/pcgen-oracle-pin.env`'s
   `PCGEN_ORACLE_SHA`) the inventory and corpus were re-derived against, so
   a reader can tell which oracle pin a given closure receipt is honest
   about.

`classify_unit()` in `shape_ledger.py` structurally never returns an
uncovered family -- it falls through to F0 (no formula content) or F8
(residual) rather than ever emitting `None` -- so on a real inventory
`unclassified_count` can never organically go non-zero. **`decisions.md`
§14a is the finding of record on this: a prior version of this gate was
accepted on a red-proof that `mock.patch`ed `shape_ledger.build_ledger` to
fabricate a row with `family: None`, which cannot occur for any real
object -- 80 fabricated units pointing at a nonexistent corpus file
returned `exit 0, PASS`.** A gate proven red only by patching the code
under test is worse than no gate (`decisions.md` §1a): it reports safety
it does not provide.

**The real, organically-reachable invariant is `join_status == "no_record"`
(`decisions.md` §14b)** -- a unit whose (book, source_file, source_line)
join finds no corpus record at all is precisely "an object no shape
covers" (AT-32-G3-001's own text), reachable through the real
`classify_unit`/`build_corpus_index` path with no patching: point a real
unit at a real-shaped but non-existent corpus location and the join
organically fails. Because 10,530 of the current 25,055-unit population
are already `no_record` (a book-onboarding backlog this cycle does not
close -- `decisions.md` §14c item 3 -- plus, as of card 15's `Kind::Skill`
landing, an entire new-kind population the corpus ingest pipeline does not
reach yet, `decisions.md` §12b), the gate cannot demand `no_record == 0`
yet without being permanently red for a reason unrelated to a regression.
Instead it enforces a **committed, evidence-gated budget**: `no_record`'s
*share* of the population may not exceed the pinned baseline share
(`NO_RECORD_BUDGET_COUNT` / `NO_RECORD_BUDGET_POPULATION` below). This is
not a pure shrink-only ratchet -- card 15 must enumerate ~9,000 more real
objects into kinds with no corpus coverage yet, and a shrink-only budget
would go redder with every unit of that mandated work. The two constants
may move, but only together with a matching, git-verifiable entry in
`no_record_budget_provenance.jsonl` naming the real commit that landed the
growth and proving it is legitimate enumeration, not drift -- see that
constant's own comment block below and `BudgetProvenanceTest` in
`scripts/tests/test_shape_coverage_standing_gate.py` for the full
mechanism. A run with no matching provenance entry -- including the
reproduction case below -- is still measured against the last committed
baseline and fails it.
`scripts/tests/test_shape_coverage_standing_gate.py` proves this
organically, by feeding real (if synthetic) units through the real
`run_gate`/`build_ledger`/`classify_unit` path with an unreachable corpus
root -- never by patching `build_ledger` or any other code under test.

Usage
-----
    python3 scripts/shape_coverage_standing_gate.py \\
        --inventory docs/work-inventory.json --corpus-root data/corpus \\
        --output artifacts/gate-3-closure-invariant/<run>.run.json

    echo '{}' | python3 scripts/shape_coverage_standing_gate.py   # fails closed

If `--inventory` is omitted and stdin is not a tty, the inventory JSON is
read from stdin (the AT-32-G3-002 verification command's exact shape). If
omitted with no piped stdin, `docs/work-inventory.json` is read instead, so
a bare interactive invocation does not hang waiting on a terminal.

Exit status is 0 only when the population is non-empty, every row's family
is set (`unclassified_count == 0`), and the piles reconcile. 1 in every
other case, with a message on stderr -- the fail-closed posture
AT-32-G3-002 requires.
"""

from __future__ import annotations

import argparse
import json
import os
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DEFAULT_INVENTORY = os.path.join(REPO_ROOT, "docs", "work-inventory.json")
DEFAULT_CORPUS_ROOT = os.path.join(REPO_ROOT, "data", "corpus")
PIN_FILE = os.path.join(REPO_ROOT, "scripts", "pcgen-oracle-pin.env")

# decisions.md §14a/§14b/§12b -- the committed no_record budget. Re-derive:
# `python3 scripts/shape_ledger.py --inventory docs/work-inventory.json
# --output /tmp/l.json && python3 -c "import json,collections;
# r=json.load(open('/tmp/l.json'))['rows']; print(collections.Counter(x
# ['join_status'] for x in r))"`.
#
# A run's no_record SHARE of its population may never exceed this baseline
# share -- compared by integer cross-multiplication (no_record_count *
# BUDGET_POPULATION > BUDGET_COUNT * population) to avoid float rounding at
# the boundary. This is Gate 3's real, organically-reachable closure
# invariant (decisions.md §14a): it catches a regression in the join, and
# it catches a fabricated object pointing at a corpus location that was
# never given a committed reason to exist there.
#
# It is NOT a "no_record must shrink every cycle" ratchet outright, because
# card 15 (decisions.md §12b) must enumerate ~9,000 real, previously
# name-only objects into 8 brand-new kinds the corpus ingest pipeline does
# not reach yet -- every one of them lands as no_record on its first
# cycle, through no defect of its own. A budget that could only shrink would
# go redder with every unit of that mandated work and would be "a gate
# pointed at the wrong thing" (SD-32 launch-readiness brief, 2026-08-22),
# exactly what decisions.md §1a forbids from the other direction: not a
# gate that cannot fail, but a gate that cannot ever be satisfied by real
# progress.
#
# Instead, the two constants below are a **committed, evidence-gated
# ratchet**: they may only move together with a matching, append-only
# entry in `no_record_budget_provenance.jsonl` (loaded by
# `read_budget_provenance()` below), each entry naming the real git commit
# that landed the population growth and the reason it is legitimate
# enumeration rather than drift. `test_shape_coverage_standing_gate.py`'s
# `BudgetProvenanceTest` mechanically enforces: (1) the constants here
# exactly match the LATEST provenance entry -- nobody can hand-edit these
# two integers without a matching logged entry: the numbers here and the
# log's tail must agree; (2) population strictly increases entry-to-entry
# -- a repin without population growth is exactly "raising the budget to
# whatever the current number is" and is refused; (3) each entry's
# no_record delta never exceeds its population delta -- the budget cannot
# widen faster than real content was added, only as fast; (4) each entry's
# `evidence_commit` is a real, reachable commit in this repo's history.
# This is what keeps card 15's mandated growth from being penalised while
# refusing to let the budget become a rubber stamp: a run with no matching
# provenance entry -- including the 80-fabricated-object reproduction,
# which lands no commit and logs no entry -- is still measured against
# the last COMMITTED baseline, and fails it.
#
# Current baseline (repin 4, 2026-08-23): no_record 21521, population
# 36028, evidence commit `004bbe8c2` (post-rebase work-inventory regen
# closing out a chain of card-15 landings: `5b2c93270` lands Kind::Ability
# via the ported per-row A/B classifier -- 4,824 new-kind units, all
# no_record, plus 112 feat units surfaced by the same classifier pass;
# `45012f6a9` narrows `is_internal_category` for Kind::ClassFeature,
# surfacing 2,617 previously-excluded class_feature rows, all no_record;
# `391993eee` gives duplicate-fallback class_feature rows a CATEGORY:-based
# identity, churning 5 ids with no net population/no_record effect;
# `9838c344d` (T12 generic class-feature roster mechanism) wires 15
# class_feature units to `text-complete`, the source of this repin's
# `departed_covered_count`). Repin 3 (2026-08-23, `8e98424eb`) landed
# no_record 13968 / population 28490. Repin 2 (2026-08-23, `d904eceb6`)
# landed no_record 10530 / population 25055. Repin 1 (2026-08-22,
# `965278926`) established the invariant itself at no_record 10419 /
# population 24914.
#
# `departed_covered_count` (repin 3 on): the strict "no_record delta never
# exceeds population delta" check (BudgetProvenanceTest) does not by itself
# account for the population's other legitimate direction of movement --
# units that finish (leave the not-done population entirely) between
# repins. A unit that was `matched`/`no_formula_tokens` and gets wired to
# completion shrinks `population` without shrinking `no_record`, which can
# make `no_record`'s delta exceed `population`'s delta even though nothing
# drained INTO no_record. Verified for repin 4, not assumed: every id in
# the repin-3-committed ledger absent from the repin-4 ledger was checked
# against the current inventory. 20 ids departed total: 15 class_feature
# ids (all `ultimate_magic`/`advanced_players_guide`/`ultimate_intrigue`
# magus/antipaladin/vigilante rows) were `no_formula_tokens` and are still
# present in `docs/work-inventory.json` at `text-complete` -- the T12
# generic class-feature roster mechanism (commit `9838c344d`) wiring
# exactly matches this count. `departed_covered_count` records those 15.
# The remaining 5 departed ids are absent entirely from the current
# inventory (2 `matched`, 2 `no_record`, 1 `no_formula_tokens`, all
# class_feature) -- the CATEGORY:-based identity fix (`391993eee`)
# rewrote their ids, so they arrive again under new ids inside this
# repin's 7,558 new-id arrivals; like repin 3's race_trait reclassification,
# this nets to zero on the check by construction and needs no credit.
# Re-derive: compare `id` sets between the previously committed
# `artifacts/gate-1-shape-closure/ledger.json` and a fresh `shape_ledger.py`
# run; for ids present in the old set and absent from the new set, look
# each up in the current `docs/work-inventory.json` -- those still present
# (by id) left via a status change (count only the ones whose OLD
# `join_status` was not `no_record`); those absent entirely were
# reclassified/renamed (kind or fallback-identity changed, so the id
# changed) and net to zero by construction.
NO_RECORD_BUDGET_COUNT = 21521
NO_RECORD_BUDGET_POPULATION = 36028

BUDGET_PROVENANCE_PATH = os.path.join(
    REPO_ROOT,
    "docs",
    "release",
    "SD-32-compute-library-and-cause-closure",
    "artifacts",
    "gate-3-closure-invariant",
    "no_record_budget_provenance.jsonl",
)


def read_budget_provenance(path: str = BUDGET_PROVENANCE_PATH) -> list[dict]:
    """Loads the append-only no_record budget repin log. Each line is one
    JSON object: {date, no_record_count, population, reason,
    evidence_commit, corpus_sha}. Returns [] if the file is missing --
    callers (the provenance test) must treat an empty log as a failure,
    never as an implicit pass, since an un-provenanced budget is exactly
    the "raise it to whatever the current number is" shape this mechanism
    exists to refuse."""
    entries: list[dict] = []
    try:
        with open(path, "r", encoding="utf-8") as fh:
            for line in fh:
                line = line.strip()
                if not line:
                    continue
                entries.append(json.loads(line))
    except OSError:
        return []
    return entries

sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import shape_ledger as SL  # noqa: E402
import coverage_ledger as CL  # noqa: E402


def read_oracle_sha(pin_file: str = PIN_FILE) -> str | None:
    """Reads `PCGEN_ORACLE_SHA` out of `scripts/pcgen-oracle-pin.env`
    without sourcing the file as shell (this is a Python script, not bash).
    Returns None if the file or the key is missing -- callers must treat a
    None the same as an unknown/unresolved citation, never as a passing
    stand-in for a real SHA."""
    try:
        with open(pin_file, "r", encoding="utf-8") as fh:
            for line in fh:
                line = line.strip()
                if line.startswith("PCGEN_ORACLE_SHA="):
                    return line.split("=", 1)[1].strip() or None
    except OSError:
        return None
    return None


def _load_inventory(inventory_path: str | None) -> dict:
    """Loads the inventory JSON. Raises OSError/json.JSONDecodeError on any
    failure -- callers must treat that as the fail-closed 'no coverage'
    case, never as an implicit zero-unit pass.

    Resolution order: an explicit --inventory path always wins; with none
    given, piped stdin (non-tty) is read (the literal AT-32-G3-002
    verification command shape); with neither, the live
    docs/work-inventory.json is read."""
    if inventory_path is not None:
        with open(inventory_path, "r", encoding="utf-8") as fh:
            return json.load(fh)
    if not sys.stdin.isatty():
        raw = sys.stdin.read()
        return json.loads(raw)
    with open(DEFAULT_INVENTORY, "r", encoding="utf-8") as fh:
        return json.load(fh)


def run_gate(
    inventory,
    corpus_root: str,
    no_record_budget_count: int = NO_RECORD_BUDGET_COUNT,
    no_record_budget_population: int = NO_RECORD_BUDGET_POPULATION,
) -> tuple[int, dict]:
    """Runs the standing gate against an already-loaded inventory dict (or
    a path -- see below) and a corpus root. Returns (exit_status, report).

    `inventory` may be a dict (already parsed -- the shape the test suite
    and any Python caller uses) or a string path (loaded here) or None
    (falls back to _load_inventory's stdin/default resolution) -- this
    keeps `main()`'s CLI plumbing and the test suite's direct calls sharing
    one code path rather than two independently-maintained ones.

    `no_record_budget_count`/`no_record_budget_population` default to the
    committed module-level baseline (decisions.md §14b) but are overridable
    -- the test suite uses a tight override to prove the invariant goes
    red on a small, real (non-mocked) population without needing tens of
    thousands of synthetic rows."""
    if not isinstance(inventory, dict):
        try:
            inventory = _load_inventory(inventory)
        except (OSError, json.JSONDecodeError) as exc:
            return 1, {
                "error": f"no coverage: cannot read/parse inventory at {inventory!r}: {exc}"
            }

    units = CL.not_done_population(inventory)
    if not units:
        return 1, {
            "error": (
                "no coverage: inventory has zero not-done units; an empty "
                "population cannot manufacture a passing gate (fail-closed "
                "posture, AT-32-G3-002)"
            )
        }

    books = {u.get("book") for u in units if u.get("book")}
    corpus_index = SL.build_corpus_index(corpus_root, books)
    # `decisions.md §20`/§17a (T9-onboarding, citation-redirect fallback):
    # without `key_index`, this gate's own no_record figure silently
    # disagrees with `shape_ledger.py`'s own CLI output the moment the
    # fallback recovers a unit -- the standing gate must reuse the SAME
    # join `build_ledger`'s caller is expected to, never a narrower one
    # that happens to compile.
    key_index = SL.build_corpus_key_index(corpus_root, books)
    # `decisions.md §20` t9-onboarding straggler wave: the cross-book
    # fallback (`build_cross_book_key_index`) recovers a unit whose real,
    # already-ingested record lives under a DIFFERENT book than any of
    # `units`' own books entirely (e.g. `occult_adventures:spell:repulsion`
    # -> the record ships under `crb`), so this walk is deliberately NOT
    # scoped to `books` -- same reasoning as `key_index` above: this gate
    # must reuse the SAME join `build_ledger`'s caller is expected to.
    cross_book_key_index = SL.build_cross_book_key_index(corpus_root)
    ledger = SL.build_ledger(units, corpus_index, key_index, cross_book_key_index)

    return evaluate_ledger(ledger, no_record_budget_count, no_record_budget_population)


def evaluate_ledger(
    ledger: dict,
    no_record_budget_count: int = NO_RECORD_BUDGET_COUNT,
    no_record_budget_population: int = NO_RECORD_BUDGET_POPULATION,
) -> tuple[int, dict]:
    """Pure evaluation step: takes an already-built ledger dict (the shape
    `shape_ledger.build_ledger` returns) and applies the three gate
    invariants, with no knowledge of how the ledger was produced. Split out
    of `run_gate` so the test suite can exercise each invariant -- including
    a deliberately malformed ledger for the sum-the-piles regression check
    -- by constructing a plain dict and calling this function directly,
    never by mocking or monkeypatching `shape_ledger.build_ledger` or any
    other code under test (decisions.md §14a: patching the thing under test
    to force red is exactly the defect this gate was reopened for)."""
    # Sum-the-piles: the per-family rollup must add back to exactly the
    # population considered -- an independent check from unclassified_count,
    # catching a build_ledger regression that silently drops rows from the
    # families dict while leaving unclassified_count reading 0.
    family_total = sum(info["count"] for info in ledger["families"].values())
    piles_reconcile = family_total == ledger["population"]
    unclassified_count = ledger["unclassified_count"]

    # decisions.md §14a/§14b -- the real, organically-reachable closure
    # invariant: no_record's SHARE of the population may not exceed the
    # committed baseline share. Integer cross-multiplication avoids float
    # rounding at the exact-baseline boundary (the real full population is
    # currently AT the baseline, not below it, so a float `>` could flip
    # sign on rounding).
    join_status_counts = ledger.get("join_status_counts", {})
    no_record_count = join_status_counts.get("no_record", 0)
    population = ledger["population"]
    no_record_budget_exceeded = (
        no_record_count * no_record_budget_population > no_record_budget_count * population
    )

    ok = piles_reconcile and unclassified_count == 0 and not no_record_budget_exceeded

    report = {
        "population": population,
        "unclassified_count": unclassified_count,
        "family_total": family_total,
        "piles_reconcile": piles_reconcile,
        "families": {fid: info["count"] for fid, info in ledger["families"].items()},
        "join_status_counts": join_status_counts,
        "no_record_count": no_record_count,
        "no_record_budget_count": no_record_budget_count,
        "no_record_budget_population": no_record_budget_population,
        "no_record_budget_exceeded": no_record_budget_exceeded,
        "corpus_sha": read_oracle_sha(),
    }
    return (0 if ok else 1), report


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument(
        "--inventory",
        default=None,
        help="path to docs/work-inventory.json; reads JSON from stdin if omitted and piped, else the live inventory",
    )
    parser.add_argument("--corpus-root", default=DEFAULT_CORPUS_ROOT, help="path to data/corpus")
    parser.add_argument("--output", help="write the full report as JSON to this path")
    parser.add_argument(
        "--no-record-budget-count",
        type=int,
        default=NO_RECORD_BUDGET_COUNT,
        help="committed no_record budget numerator (decisions.md §14b default)",
    )
    parser.add_argument(
        "--no-record-budget-population",
        type=int,
        default=NO_RECORD_BUDGET_POPULATION,
        help="committed no_record budget denominator (decisions.md §14b default)",
    )
    args = parser.parse_args(argv)

    status, report = run_gate(
        args.inventory,
        args.corpus_root,
        no_record_budget_count=args.no_record_budget_count,
        no_record_budget_population=args.no_record_budget_population,
    )

    if "error" in report:
        print(report["error"], file=sys.stderr)
        return status

    print(f"population (not-done units considered): {report['population']}")
    print(f"unclassified: {report['unclassified_count']}")
    print(
        f"piles reconcile: {report['piles_reconcile']} "
        f"({report['family_total']} families-total == {report['population']} population)"
    )
    jsc = report.get("join_status_counts", {})
    print(
        "join-status split (decisions.md §14b): "
        f"matched={jsc.get('matched', 0)} "
        f"no_formula_tokens={jsc.get('no_formula_tokens', 0)} "
        f"no_record={report['no_record_count']}"
    )
    print(
        f"no_record budget: {report['no_record_count']}/{report['population']} vs. baseline "
        f"{report['no_record_budget_count']}/{report['no_record_budget_population']} -- "
        f"exceeded: {report['no_record_budget_exceeded']}"
    )
    print(f"corpus SHA: {report.get('corpus_sha') or 'unknown'}")
    print()
    print("family rollup:")
    for fid, count in sorted(report["families"].items()):
        print(f"  {fid:<4} {count:>7}")

    if status != 0:
        print(
            "shape-coverage-standing-gate: FAIL — an object appears that no "
            "shape covers, the per-family piles do not reconcile to the "
            "population, or the no_record share exceeds its committed "
            "budget (decisions.md §14a/§14b)",
            file=sys.stderr,
        )

    if args.output:
        os.makedirs(os.path.dirname(args.output) or ".", exist_ok=True)
        with open(args.output, "w", encoding="utf-8") as fh:
            json.dump(report, fh, indent=2)
            fh.write("\n")

    return status


if __name__ == "__main__":
    sys.exit(main())
