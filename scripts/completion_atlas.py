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
    6. a bucket whose definition does not cite the source that emits the
       evidence strings it keys on -- or whose citation no longer resolves,
       or whose cited *content* changed (content, not path/line --
       `risks-and-open-questions.md §10`)

Each `BUCKET_DEFINITIONS` entry carries a `citation` that is a CONTENT
ANCHOR (SD-35 `AT-35-E1-002`): `{file, context_fn, anchor}` -- the exact
source line(s) that emit the marker/status string the bucket keys on, inside
a named function of `src/bin/v06_work_inventory.rs`, resolved by search on
every run (`resolve_content_anchor`). `shape_engine_boundary.py` and
`missing_engine_tables.py` import the same resolver.

Output modes:

    --check                 the fail-closed partition + the six conditions
    --by-book               one row per book, bucket counts and percentages
    --by-kind [--book B]    one row per kind, bucket counts and percentages
    --by-evidence [--book B] [--bucket X]
                            per bucket, the distinct evidence strings and
                            how many units carry each (the sub-cause census)
    --book B --check        exit 0 only when that book's non-DONE buckets are 0
"""

from __future__ import annotations

import argparse
import collections
import json
import os
import re
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

# --- content anchors (SD-35 AT-35-E1-002) -----------------------------------
#
# Every bucket cites the source that emits the evidence/status string it keys
# on as a CONTENT ANCHOR, not a `file:line` pin: `{file, context_fn, anchor}`
# where `anchor` is the exact source line(s) -- stripped of indentation --
# that must appear, consecutively and exactly once, inside the body of the
# named function. `resolve_content_anchor` searches for it at check time, so
# a refactor that moves the code keeps the citation green and any change to
# the cited content (or a second copy of it, or the function vanishing)
# still fails closed -- SD-34 AT-34-E1-002 condition 6, preserved.
#
# Why: the `file:line` pins this block carried before were re-derived by hand
# in eleven separate SD-34 waves (32, 33, 35, 38, 40, 44, 45, 46, 47, 48, 50),
# went silently stale twice on a doc-comment that happened to contain the
# bare marker substring, and were found drifted at HEAD by wave 51 -- every
# edit to `v06_work_inventory.rs` shifted them. `git log -p` on this file
# keeps that history; it no longer needs to live here. The multi-line anchors
# below are the minimum that is unique inside the named function (e.g.
# `status: "grounded",` appears twice in `simple_kind_verdict`, so DONE's
# anchor carries the `if let Some(bonus) = grounded_magnitude {` line above
# its construction site).

BUCKET_DEFINITIONS = {
    "DONE": {
        "meaning": "nothing remains",
        "clears": "—",
        "evidence_source": "src/bin/v06_work_inventory.rs (status in {grounded, text-complete})",
        # The `grounded` construction site in `simple_kind_verdict` (one of
        # the two DONE statuses): the fixture-verified magnitude arm.
        "citation": {
            "file": _ENGINE_SRC,
            "context_fn": "simple_kind_verdict",
            "anchor": [
                "if let Some(bonus) = grounded_magnitude {",
                "return Verdict {",
                'status: "grounded",',
            ],
        },
    },
    "A": {
        "meaning": "engine has no table for this kind",
        "clears": "building the table (Epic 2)",
        "evidence_source": (
            "src/bin/v06_work_inventory.rs "
            "(evidence contains 'has_no_engine_table')"
        ),
        # The `companion` arm emitting the bucket-A marker in `classify`
        # (the same site `missing_engine_tables.py` cites for `companion`).
        "citation": {
            "file": _ENGINE_SRC,
            "context_fn": "classify",
            "anchor": [
                'Kind::Companion => engine_does_not_hold("companion_content_has_no_engine_table"),',
            ],
        },
    },
    "B": {
        "meaning": "table exists, record not in it",
        "clears": "placing the record (Epic 3/4)",
        "evidence_source": (
            "src/bin/v06_work_inventory.rs "
            "(evidence contains 'not_held_by_engine' / 'absent_from' / 'not_modelled')"
        ),
        # The class-feature option-pool `not_held_by_engine` emission in
        # `classify`.
        "citation": {
            "file": _ENGINE_SRC,
            "context_fn": "classify",
            "anchor": [
                'return engine_does_not_hold("class_feature_option_pool_record_not_held_by_engine");',
            ],
        },
    },
    "C": {
        "meaning": "held and computed, never surfaced",
        "clears": "wiring the display/explanation path (Epic 3)",
        "evidence_source": (
            "src/bin/v06_work_inventory.rs "
            "(evidence contains 'explanation_id' / 'diagnostic')"
        ),
        # The class-feature fallthrough naming both `explanation_id` and
        # `diagnostic` in `classify`.
        "citation": {
            "file": _ENGINE_SRC,
            "context_fn": "classify",
            "anchor": [
                'engine_does_not_hold("no_explanation_id_and_no_diagnostic_names_this_feature")',
            ],
        },
    },
    "D": {
        "meaning": "other engine gap (sub-causes enumerated, never a shrug)",
        "clears": "per named sub-cause",
        "evidence_source": "src/bin/v06_work_inventory.rs (status == engine-does-not-hold, no other bucket matched)",
        # The shared `engine_does_not_hold` closure in `classify` that stamps
        # `status: "engine-does-not-hold"` for every arm that falls through
        # A/B/C -- this IS the D fallthrough.
        "citation": {
            "file": _ENGINE_SRC,
            "context_fn": "classify",
            "anchor": [
                "let engine_does_not_hold = |evidence: &str| Verdict {",
                'status: "engine-does-not-hold",',
            ],
        },
    },
    "M": {
        "meaning": "magnitude ingested, never computed or applied",
        "clears": "running the compute path (shape engine)",
        "evidence_source": "src/bin/v06_work_inventory.rs (status == ingested-magnitude)",
        # The `ingested-magnitude` construction site in `simple_kind_verdict`
        # (a held record whose magnitude the compute path has not run).
        "citation": {
            "file": _ENGINE_SRC,
            "context_fn": "simple_kind_verdict",
            "anchor": [
                'status: "ingested-magnitude",',
                'evidence: format!("{kind_label}_table_holds_record_magnitude_not_yet_computed"),',
            ],
        },
    },
    "V": {
        "meaning": "verified by proxy, never by the oracle",
        "clears": "the SD-33 oracle harness (scripts/oracle_harness/)",
        "evidence_source": "src/bin/v06_work_inventory.rs (status in {literal-verified, fixture-verified})",
        # The `literal-verified` stamp in `apply_done_rung_stamps` (one of the
        # two V statuses).
        "citation": {
            "file": _ENGINE_SRC,
            "context_fn": "apply_done_rung_stamps",
            "anchor": [
                'item.verdict.status = "literal-verified";',
            ],
        },
    },
    "U": {
        "meaning": "instrument cannot express a verdict",
        "clears": "instrument correction",
        "evidence_source": "src/bin/v06_work_inventory.rs (status == unmeasurable)",
        # The first `unmeasurable` construction site in `classify`: a feat
        # whose served description is an upstream marker string, not prose.
        "citation": {
            "file": _ENGINE_SRC,
            "context_fn": "classify",
            "anchor": [
                'status: "unmeasurable",',
                'evidence: "feat_served_description_is_a_placeholder_marker_not_prose".to_string(),',
            ],
        },
    },
    "X": {
        "meaning": "deferred with a stated reason",
        "clears": "revisiting the stated condition",
        "evidence_source": "src/bin/v06_work_inventory.rs (status == deferred-with-reason)",
        # The first `deferred-with-reason` construction site in `classify`:
        # the Ultimate Campaign feat-table diagnostic.
        "citation": {
            "file": _ENGINE_SRC,
            "context_fn": "classify",
            "anchor": [
                'status: "deferred-with-reason",',
                'evidence: "engine_diagnostic:ultimate_campaign::feat_tables::DEFERRED_WITH_REASON"',
            ],
        },
    },
    "Z": {
        "meaning": "not started",
        "clears": "ordinary work",
        "evidence_source": "src/bin/v06_work_inventory.rs (status == not-started)",
        # The only `not-started` construction site: a book with no compiled
        # rule set, in `classify`.
        "citation": {
            "file": _ENGINE_SRC,
            "context_fn": "classify",
            "anchor": [
                'status: "not-started",',
                'evidence: "no_compiled_rule_set_for_book".to_string(),',
            ],
        },
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
    # `decisions.md §19`, extending `§17`'s disposition principle from bucket
    # U to bucket V: a unit the consolidated bucket-V oracle ledger
    # (AT-34-E3-005) resolved to `agree` (a real oracle round-trip matched)
    # or `unverifiable` (a real, named reason no verdict can be reached) is
    # dispositioned -- nothing remains -- and leaves bucket V for DONE. A
    # `disagree` verdict is never mapped to either of these two statuses
    # (`v06_work_inventory.rs::apply_bucket_v_oracle_disposition_stamps`), so
    # it can never reach this branch.
    if status in ("oracle-agree", "oracle-unverifiable"):
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


def read_source_lines(rel_path: str, repo_root: str = REPO_ROOT) -> "list[str] | None":
    """The cited file's lines (newlines stripped), or None when it is absent."""
    abs_path = os.path.join(repo_root, rel_path)
    try:
        with open(abs_path, "r", encoding="utf-8") as fh:
            return fh.read().split("\n")
    except OSError:
        return None


# A Rust function definition line: optional visibility / qualifiers, then
# `fn <name>` followed by its generics or parameter list. `classify(` is
# matched; `classify_class_feature_delta(` is not.
_FN_DEF_RE = re.compile(
    r"^(?P<indent>\s*)(?:pub(?:\([^)]*\))?\s+)?(?:(?:const|async|unsafe|extern\s+\"[^\"]*\")\s+)*"
    r"fn\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*[<(]"
)


def _fn_extents(lines: list, fn_name: str) -> list:
    """Every `(start, end)` 0-indexed inclusive line span whose `fn fn_name`
    definition opens it and whose closing `}` sits at the definition's own
    indentation -- rustfmt's invariant, which `v06_work_inventory.rs` is
    formatted under. A body that never closes runs to end of file."""
    extents = []
    for i, line in enumerate(lines):
        m = _FN_DEF_RE.match(line)
        if not m or m.group("name") != fn_name:
            continue
        indent = m.group("indent")
        end = len(lines) - 1
        for j in range(i + 1, len(lines)):
            if lines[j] == indent + "}":
                end = j
                break
        extents.append((i, end))
    return extents


def resolve_content_anchor(citation: dict, lines: "list[str] | None" = None,
                           repo_root: str = REPO_ROOT) -> dict:
    """Resolve a content anchor `{file, context_fn, anchor}` by search.

    `anchor` is one line or a list of consecutive lines, compared after
    stripping indentation. The anchor must occur EXACTLY ONCE inside the body
    of `context_fn` (searched across every definition of that name). Returns
    `{"ok": True, "line": <1-indexed first line>, "end_line": <1-indexed last
    line>, "fn_line": <1-indexed fn definition line>, "source": [<the matched
    lines as written>]}` or `{"ok": False, "reason": <why>}`.

    A refactor that MOVES the function or the block keeps the anchor green; a
    change to any cited line, a second copy of the block inside the function,
    or the function/file disappearing fails closed (SD-34 AT-34-E1-002
    condition 6, preserved by SD-35 AT-35-E1-002).

    `lines` lets a caller (a test) resolve against text it supplies instead
    of the file on disk."""
    rel_path = citation.get("file")
    fn_name = citation.get("context_fn")
    anchor = citation.get("anchor")
    if isinstance(anchor, str):
        anchor = [anchor]
    if not rel_path or not fn_name or not anchor:
        return {"ok": False, "reason": "citation is missing file, context_fn, or anchor"}
    wanted = [a.strip() for a in anchor]
    if any(not w for w in wanted):
        return {"ok": False, "reason": "anchor contains an empty line"}

    if lines is None:
        lines = read_source_lines(rel_path, repo_root)
    if lines is None:
        return {"ok": False, "reason": f"{rel_path} does not resolve (file not found)"}

    extents = _fn_extents(lines, fn_name)
    if not extents:
        return {"ok": False, "reason": f"{rel_path}: fn {fn_name} does not resolve (no definition found)"}

    hits = []
    n = len(wanted)
    for start, end in extents:
        for i in range(start, end - n + 2):
            if all(lines[i + k].strip() == wanted[k] for k in range(n)):
                hits.append((start, i))
    if not hits:
        return {
            "ok": False,
            "reason": f"{rel_path}: fn {fn_name} no longer contains {wanted!r}",
        }
    if len(hits) > 1:
        return {
            "ok": False,
            "reason": (
                f"{rel_path}: fn {fn_name} contains {wanted!r} {len(hits)} times "
                f"(lines {[h[1] + 1 for h in hits]}) -- ambiguous, lengthen the anchor"
            ),
        }
    fn_start, i = hits[0]
    return {
        "ok": True,
        "line": i + 1,
        "end_line": i + n,
        "fn_line": fn_start + 1,
        "source": lines[i:i + n],
    }


def _citation_failures(definitions: dict = BUCKET_DEFINITIONS) -> list:
    """Condition 6. Every bucket must cite, as a content anchor, the source
    that emits the evidence string it keys on, and that anchor must resolve
    by search at HEAD -- exactly once, inside the named function, with the
    cited lines' CONTENT unchanged (`risks-and-open-questions.md §10`)."""
    failures = []
    for b in BUCKET_ORDER:
        citation = definitions.get(b, {}).get("citation")
        if not citation:
            failures.append(f"{b}: no citation")
            continue
        resolved = resolve_content_anchor(citation)
        if not resolved["ok"]:
            failures.append(f"{b}: {resolved['reason']}")
    return failures


def resolved_citations(definitions: dict = BUCKET_DEFINITIONS) -> dict:
    """Each bucket's citation with the line it resolved to at check time
    (`resolved_line` is derived, never pinned; None when unresolved)."""
    out = {}
    for b in BUCKET_ORDER:
        citation = definitions.get(b, {}).get("citation")
        if not citation:
            out[b] = None
            continue
        resolved = resolve_content_anchor(citation)
        out[b] = dict(citation)
        out[b]["resolved_line"] = resolved["line"] if resolved["ok"] else None
    return out


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
        citations = resolved_citations()
        artifact = {
            "population": population,
            "derived_at": _head_sha(),
            "buckets": {
                b: {
                    "count": counts.get(b, 0),
                    "meaning": BUCKET_DEFINITIONS[b]["meaning"],
                    "clears": BUCKET_DEFINITIONS[b]["clears"],
                    "evidence_source": BUCKET_DEFINITIONS[b]["evidence_source"],
                    "citation": citations[b],
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


def _bucket_row(counts: "collections.Counter", total: int) -> str:
    return " ".join(
        f"{b}={counts.get(b, 0)}({(counts.get(b, 0) / total * 100 if total else 0):.1f}%)"
        for b in BUCKET_ORDER
    )


def cmd_by_book(args) -> int:
    inv = _load_inventory()
    units = inv["units"]
    books = sorted({u.get("book") for u in units if u.get("book")})
    for book in books:
        result = partition(units, book=book)
        print(f"{book} (n={result['examined']}): {_bucket_row(result['counts'], result['examined'])}")
    return 0


def by_kind(units: list, book: "str | None" = None) -> dict:
    """`{kind: {"examined": n, "counts": Counter}}` -- the same partition as
    `partition()`, grouped by the unit's `kind` (a missing kind groups under
    `None` so no unit is dropped from the census)."""
    groups: "dict[str | None, list]" = collections.defaultdict(list)
    for unit in units:
        if book is not None and unit.get("book") != book:
            continue
        groups[unit.get("kind")].append(unit)
    out = {}
    for kind, group in groups.items():
        result = partition(group)
        out[kind] = {"examined": result["examined"], "counts": result["counts"]}
    return out


def cmd_by_kind(args) -> int:
    inv = _load_inventory()
    rows = by_kind(inv["units"], book=args.book)
    for kind in sorted(rows, key=lambda k: (k is None, k or "")):
        r = rows[kind]
        print(f"{kind} (n={r['examined']}): {_bucket_row(r['counts'], r['examined'])}")
    return 0


def by_evidence(units: list, book: "str | None" = None,
                bucket: "str | None" = None) -> dict:
    """`{bucket: Counter(evidence -> units)}` for every bucket the units land
    in (or only `bucket` when given) -- `_sub_causes` generalised from D/U to
    the whole partition. A unit with no evidence counts under `None`."""
    out: "dict[str, collections.Counter]" = collections.defaultdict(collections.Counter)
    for unit in units:
        if book is not None and unit.get("book") != book:
            continue
        b = _bucket_of(unit)
        if b is None or (bucket is not None and b != bucket):
            continue
        out[b][unit.get("evidence")] += 1
    return dict(out)


def cmd_by_evidence(args) -> int:
    if args.bucket is not None and args.bucket not in BUCKET_ORDER:
        print(f"unknown bucket {args.bucket!r}; one of {BUCKET_ORDER}", file=sys.stderr)
        return 2
    inv = _load_inventory()
    rows = by_evidence(inv["units"], book=args.book, bucket=args.bucket)
    for b in BUCKET_ORDER:
        if b not in rows:
            continue
        counter = rows[b]
        print(f"bucket={b} population={sum(counter.values())} distinct_evidence={len(counter)}")
        for evidence, n in counter.most_common():
            print(f"  {n:>6}  {evidence}")
    return 0


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--by-book", action="store_true")
    parser.add_argument("--by-kind", action="store_true")
    parser.add_argument("--by-evidence", action="store_true")
    parser.add_argument("--book", default=None)
    parser.add_argument("--bucket", default=None, help="with --by-evidence: restrict to one bucket")
    args = parser.parse_args(argv)

    if args.by_book:
        return cmd_by_book(args)
    if args.by_kind:
        return cmd_by_kind(args)
    if args.by_evidence:
        return cmd_by_evidence(args)
    if args.check:
        return cmd_check(args)
    parser.print_help()
    return 2


if __name__ == "__main__":
    sys.exit(main())
