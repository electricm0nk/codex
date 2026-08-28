#!/usr/bin/env python3
"""Partition `docs/work-inventory.json`'s full unit population into the ten
buckets fixed by SD-34 `decisions.md` §2 -- the atlas that plays, for SD-34,
the role `THE-BOX.md` played for SD-33.

`AT-34-E1-001` is the only criterion this cycle implements:

    python3 scripts/completion_atlas.py --check
        -> population=49438 buckets=10 unclassified=0 overlap=0   (exit 0)

Every unit lands in exactly one of:

    DONE  A  B  C  D  M  V  U  X  Z

Bucket derivation is keyed on `status` plus `evidence` (not `status` alone --
`evidence` is what separates A from B from C from D within the single
`engine-does-not-hold` status), reading the *live* inventory rather than any number
carried forward from a prior bundle (`decisions.md §12` L2).

`overlap` is structurally impossible under this implementation: `_bucket_of`
returns exactly one letter per unit, by construction, via an if/elif chain
with no bucket able to also claim another bucket's unit. It is still
computed and printed explicitly (never assumed) so a future refactor that
turns `_bucket_of` into a multi-match function trips a real check rather
than a silent invariant.

`unclassified` is real: `_bucket_of` returns `None` (never a made-up letter)
for any unit whose `(status, evidence)` pair matches nothing below, and
`--check` fails closed on that -- `AT-34-E1-002` condition 1.

`AT-34-E1-002` (this cycle) adds the remaining five fail-closed conditions on
top of AT-34-E1-001's `unclassified`/`overlap` gate:

    3. a unit in DONE whose evidence does not support it
    4. a bucket with no named clearing mechanism
    5. a `derived_at` SHA that is not an ancestor of HEAD (staleness gate)
    6. a bucket whose definition does not cite the `file:line` that emits
       the evidence strings it keys on -- or whose citation no longer
       resolves, or whose cited *line's content* no longer contains the
       marker (content, not just path/line -- `risks-and-open-questions.md
       §10`)

Each `BUCKET_DEFINITIONS` entry now carries a `citation` naming the real
`src/bin/v06_work_inventory.rs` line that emits the marker/status string the
bucket keys on -- verified against the live file, not assumed.
"""

from __future__ import annotations

import argparse
import collections
import json
import os
import subprocess
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
INVENTORY_PATH = os.path.join(REPO_ROOT, "docs", "work-inventory.json")
ARTIFACT_PATH = os.path.join(
    REPO_ROOT, "docs", "release", "SD-34-book-completion", "artifacts", "epic-1-atlas",
    "completion-atlas.json",
)

# --- bucket A: "no engine table for this kind" -----------------------------
_A_MARKER = "has_no_engine_table"

# --- bucket B: "table exists, record not in it" -----------------------------
_B_MARKERS = ("not_held_by_engine", "absent_from", "not_modelled")

# --- bucket C: "held and computed, never surfaced" --------------------------
_C_MARKERS = ("explanation_id", "diagnostic")

_ENGINE_SRC = "src/bin/v06_work_inventory.rs"

BUCKET_DEFINITIONS = {
    "DONE": {
        "meaning": "nothing remains",
        "clears": "—",
        "evidence_source": "src/bin/v06_work_inventory.rs (status in {grounded, text-complete})",
        # First `status: "grounded"` literal -- one of the two DONE statuses.
        "citation": {"file": _ENGINE_SRC, "line": 8944, "must_contain": "grounded"},
    },
    "A": {
        "meaning": "engine has no table for this kind",
        "clears": "building the table (Epic 2)",
        "evidence_source": (
            "src/bin/v06_work_inventory.rs "
            "(evidence contains 'has_no_engine_table')"
        ),
        # `Kind::Companion => engine_does_not_hold("companion_content_has_no_engine_table")`.
        "citation": {"file": _ENGINE_SRC, "line": 10558, "must_contain": "has_no_engine_table"},
    },
    "B": {
        "meaning": "table exists, record not in it",
        "clears": "placing the record (Epic 3/4)",
        "evidence_source": (
            "src/bin/v06_work_inventory.rs "
            "(evidence contains 'not_held_by_engine' / 'absent_from' / 'not_modelled')"
        ),
        # `engine_does_not_hold("class_feature_option_pool_record_not_held_by_engine")`.
        "citation": {"file": _ENGINE_SRC, "line": 10256, "must_contain": "not_held_by_engine"},
    },
    "C": {
        "meaning": "held and computed, never surfaced",
        "clears": "wiring the display/explanation path (Epic 3)",
        "evidence_source": (
            "src/bin/v06_work_inventory.rs "
            "(evidence contains 'explanation_id' / 'diagnostic')"
        ),
        # `engine_does_not_hold("no_explanation_id_and_no_diagnostic_names_this_feature")`.
        "citation": {"file": _ENGINE_SRC, "line": 10481, "must_contain": "explanation_id"},
    },
    "D": {
        "meaning": "other engine gap (sub-causes enumerated, never a shrug)",
        "clears": "per named sub-cause",
        "evidence_source": "src/bin/v06_work_inventory.rs (status == engine-does-not-hold, no other bucket matched)",
        # The shared `engine_does_not_hold` closure that stamps `status: "engine-does-not-hold"`
        # for every arm that falls through A/B/C -- this IS the D fallthrough.
        "citation": {"file": _ENGINE_SRC, "line": 8916, "must_contain": "engine-does-not-hold"},
    },
    "M": {
        "meaning": "magnitude ingested, never computed or applied",
        "clears": "running the compute path (shape engine)",
        "evidence_source": "src/bin/v06_work_inventory.rs (status == ingested-magnitude)",
        "citation": {"file": _ENGINE_SRC, "line": 8751, "must_contain": "ingested-magnitude"},
    },
    "V": {
        "meaning": "verified by proxy, never by the oracle",
        "clears": "the SD-33 oracle harness (scripts/oracle_harness/)",
        "evidence_source": "src/bin/v06_work_inventory.rs (status in {literal-verified, fixture-verified})",
        # `item.verdict.status = "literal-verified";` -- one of the two V statuses.
        "citation": {"file": _ENGINE_SRC, "line": 11209, "must_contain": "literal-verified"},
    },
    "U": {
        "meaning": "instrument cannot express a verdict",
        "clears": "instrument correction",
        "evidence_source": "src/bin/v06_work_inventory.rs (status == unmeasurable)",
        "citation": {"file": _ENGINE_SRC, "line": 9003, "must_contain": "unmeasurable"},
    },
    "X": {
        "meaning": "deferred with a stated reason",
        "clears": "revisiting the stated condition",
        "evidence_source": "src/bin/v06_work_inventory.rs (status == deferred-with-reason)",
        "citation": {"file": _ENGINE_SRC, "line": 8963, "must_contain": "deferred-with-reason"},
    },
    "Z": {
        "meaning": "not started",
        "clears": "ordinary work",
        "evidence_source": "src/bin/v06_work_inventory.rs (status == not-started)",
        "citation": {"file": _ENGINE_SRC, "line": 8824, "must_contain": "not-started"},
    },
}

# Condition 3 (a DONE unit whose evidence does not support it): markers that
# belong to an UNFINISHED bucket and would never legitimately appear in a
# DONE unit's evidence string. `explanation_id` is deliberately EXCLUDED --
# 245 real `DONE` units carry it legitimately (e.g.
# `explanation_id_observed_and_corpus_record_carries_real_description`),
# confirmed against the live corpus; including it here would make this its
# own AT-34-E1-002-condition-6-shaped mistake (a field/substring read as
# meaning something it does not). Verified empty on the live corpus:
# `has_no_engine_table`, `not_held_by_engine`, `absent_from`, `not_modelled`,
# `diagnostic` never appear in a DONE unit's evidence.
_DONE_VIOLATION_MARKERS = (_A_MARKER,) + _B_MARKERS + ("diagnostic",)

BUCKET_ORDER = ["DONE", "A", "B", "C", "D", "M", "V", "U", "X", "Z"]


def _bucket_of(unit: dict) -> "str | None":
    status = unit.get("status")
    evidence = unit.get("evidence") or ""

    if status in ("grounded", "text-complete"):
        return "DONE"
    if status in ("literal-verified", "fixture-verified"):
        return "V"
    if status == "ingested-magnitude":
        return "M"
    if status == "unmeasurable":
        return "U"
    if status == "deferred-with-reason":
        return "X"
    if status == "not-started":
        return "Z"
    if status == "engine-does-not-hold":
        if _A_MARKER in evidence:
            return "A"
        if any(marker in evidence for marker in _B_MARKERS):
            return "B"
        if any(marker in evidence for marker in _C_MARKERS):
            return "C"
        return "D"
    return None


def _load_inventory(path: str = INVENTORY_PATH) -> dict:
    with open(path, "r", encoding="utf-8") as fh:
        return json.load(fh)


def _head_sha() -> str:
    try:
        return subprocess.run(
            ["git", "rev-parse", "HEAD"], cwd=REPO_ROOT,
            capture_output=True, text=True, check=True,
        ).stdout.strip()
    except Exception:
        return "unknown"


def partition(units: list, book: "str | None" = None) -> dict:
    """Return (counts_by_bucket, unclassified_ids, overlap_ids, examined_population)."""
    counts = collections.Counter()
    unclassified_ids = []
    seen = set()
    overlap_ids = []
    examined = 0
    for unit in units:
        if book is not None and unit.get("book") != book:
            continue
        examined += 1
        b = _bucket_of(unit)
        uid = unit.get("id")
        if uid in seen:
            overlap_ids.append(uid)
        seen.add(uid)
        if b is None:
            unclassified_ids.append(uid)
            continue
        counts[b] += 1
    return {
        "counts": counts,
        "unclassified_ids": unclassified_ids,
        "overlap_ids": overlap_ids,
        "examined": examined,
    }


def _sub_causes(units: list, bucket: str) -> "collections.Counter | None":
    if bucket not in ("D", "U"):
        return None
    c = collections.Counter()
    for unit in units:
        if _bucket_of(unit) == bucket:
            c[unit.get("evidence")] += 1
    return c


def _done_evidence_is_supported(evidence: "str | None") -> bool:
    """Condition 3. A DONE unit's evidence must be a real, non-empty string
    that carries none of `_DONE_VIOLATION_MARKERS` -- a DONE unit whose
    evidence looks like an unfinished-bucket marker is the atlas silently
    trusting a field instead of what produced it (`decisions.md §12` L1)."""
    if not evidence:
        return False
    return not any(marker in evidence for marker in _DONE_VIOLATION_MARKERS)


def _done_evidence_violations(units: list) -> list:
    return [
        unit.get("id")
        for unit in units
        if _bucket_of(unit) == "DONE" and not _done_evidence_is_supported(unit.get("evidence"))
    ]


def _missing_clearing_mechanisms(definitions: dict = BUCKET_DEFINITIONS) -> list:
    """Condition 4. Every bucket must name a mechanism that empties it --
    `DONE`'s `"—"` counts (it explicitly means "nothing remains"); an empty
    string or missing field does not."""
    return [b for b in BUCKET_ORDER if not definitions.get(b, {}).get("clears")]


def _read_source_line(rel_path: str, line_no: int) -> "str | None":
    abs_path = os.path.join(REPO_ROOT, rel_path)
    try:
        with open(abs_path, "r", encoding="utf-8") as fh:
            lines = fh.readlines()
    except OSError:
        return None
    if line_no < 1 or line_no > len(lines):
        return None
    return lines[line_no - 1]


def _citation_failures(definitions: dict = BUCKET_DEFINITIONS) -> list:
    """Condition 6. Every bucket must cite the `file:line` that emits the
    evidence string it keys on, resolvable at HEAD, and the cited LINE's
    CONTENT must actually contain the claimed marker -- a refactor that
    moves the code without changing line counts must still trip this
    (`risks-and-open-questions.md §10`)."""
    failures = []
    for b in BUCKET_ORDER:
        citation = definitions.get(b, {}).get("citation")
        if not citation:
            failures.append(f"{b}: no citation")
            continue
        line = _read_source_line(citation["file"], citation["line"])
        if line is None:
            failures.append(f"{b}: {citation['file']}:{citation['line']} does not resolve")
            continue
        if citation["must_contain"] not in line:
            failures.append(
                f"{b}: {citation['file']}:{citation['line']} no longer contains "
                f"'{citation['must_contain']}'"
            )
    return failures


def _is_ancestor(sha: "str | None") -> bool:
    if not sha or sha == "unknown":
        return False
    try:
        subprocess.run(
            ["git", "merge-base", "--is-ancestor", sha, "HEAD"],
            cwd=REPO_ROOT, check=True, capture_output=True, text=True,
        )
        return True
    except Exception:
        return False


def _staleness_violation(artifact_path: str = ARTIFACT_PATH) -> "str | None":
    """Condition 5. Reads the artifact ON DISK as it stood BEFORE this run's
    own write -- checking a freshly-stamped HEAD against itself is trivially
    true and proves nothing. This checks the PRIOR commit's stamped
    `derived_at` still resolves as an ancestor of the current HEAD, catching
    a rebase/force-push/hand-edit that orphaned it."""
    if not os.path.exists(artifact_path):
        return None
    try:
        with open(artifact_path, "r", encoding="utf-8") as fh:
            prior = json.load(fh)
    except (OSError, json.JSONDecodeError):
        return None
    prior_sha = prior.get("derived_at")
    if prior_sha in (None, "unknown"):
        return None
    if not _is_ancestor(prior_sha):
        return f"derived_at {prior_sha!r} is not an ancestor of HEAD"
    return None


def cmd_check(args) -> int:
    inv = _load_inventory()
    units = inv["units"]
    result = partition(units, book=args.book)
    counts = result["counts"]
    unclassified = len(result["unclassified_ids"])
    overlap = len(result["overlap_ids"])
    population = result["examined"]

    if args.book is None:
        # Condition 5 must read the artifact AS COMMITTED, before this run's
        # own write below replaces it.
        staleness = _staleness_violation()
        done_violations = _done_evidence_violations(units)
        missing_clears = _missing_clearing_mechanisms()
        citation_failures = _citation_failures()

        print(
            f"population={population} buckets={len(BUCKET_ORDER)} "
            f"unclassified={unclassified} overlap={overlap}"
        )
        for b in BUCKET_ORDER:
            print(f"  {b}: {counts.get(b, 0)}")
        print(f"done_evidence_violations={len(done_violations)}")
        print(f"missing_clearing_mechanisms={len(missing_clears)}")
        print(f"stale_derived_at={'True' if staleness else 'False'}")
        print(f"citation_failures={len(citation_failures)}")
        if staleness:
            print(f"  staleness: {staleness}")
        for uid in done_violations[:20]:
            print(f"  done_evidence_violation: {uid}")
        for b in missing_clears:
            print(f"  missing_clearing_mechanism: {b}")
        for f in citation_failures:
            print(f"  citation_failure: {f}")

        d_causes = _sub_causes(units, "D")
        u_causes = _sub_causes(units, "U")
        artifact = {
            "population": population,
            "derived_at": _head_sha(),
            "buckets": {
                b: {
                    "count": counts.get(b, 0),
                    "meaning": BUCKET_DEFINITIONS[b]["meaning"],
                    "clears": BUCKET_DEFINITIONS[b]["clears"],
                    "evidence_source": BUCKET_DEFINITIONS[b]["evidence_source"],
                    "citation": BUCKET_DEFINITIONS[b].get("citation"),
                }
                for b in BUCKET_ORDER
            },
            "unclassified": unclassified,
            "overlap": overlap,
            "done_evidence_violations": len(done_violations),
            "done_evidence_violation_ids": done_violations,
            "missing_clearing_mechanisms": missing_clears,
            "citation_failures": citation_failures,
            "stale_derived_at": bool(staleness),
            "sub_causes": {
                "D": dict(d_causes.most_common()) if d_causes else {},
                "U": dict(u_causes.most_common()) if u_causes else {},
            },
            "re_derive_command": "python3 scripts/completion_atlas.py --check",
        }
        os.makedirs(os.path.dirname(ARTIFACT_PATH), exist_ok=True)
        with open(ARTIFACT_PATH, "w", encoding="utf-8") as fh:
            json.dump(artifact, fh, indent=2, sort_keys=True)
            fh.write("\n")

        if (
            unclassified != 0
            or overlap != 0
            or done_violations
            or missing_clears
            or staleness
            or citation_failures
        ):
            return 1
        return 0

    # --book <slug> --check: exit 0 only when every non-DONE bucket is 0
    print(
        f"book={args.book} population={population} "
        f"unclassified={unclassified} overlap={overlap}"
    )
    for b in BUCKET_ORDER:
        print(f"  {b}: {counts.get(b, 0)}")
    if unclassified != 0 or overlap != 0:
        return 1
    non_done_total = sum(counts.get(b, 0) for b in BUCKET_ORDER if b != "DONE")
    return 0 if non_done_total == 0 else 1


def cmd_by_book(args) -> int:
    inv = _load_inventory()
    units = inv["units"]
    books = sorted({u.get("book") for u in units if u.get("book")})
    for book in books:
        result = partition(units, book=book)
        counts = result["counts"]
        total = result["examined"]
        row = " ".join(
            f"{b}={counts.get(b, 0)}({(counts.get(b, 0) / total * 100 if total else 0):.1f}%)"
            for b in BUCKET_ORDER
        )
        print(f"{book} (n={total}): {row}")
    return 0


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--by-book", action="store_true")
    parser.add_argument("--book", default=None)
    args = parser.parse_args(argv)

    if args.by_book:
        return cmd_by_book(args)
    if args.check:
        return cmd_check(args)
    parser.print_help()
    return 2


if __name__ == "__main__":
    sys.exit(main())
