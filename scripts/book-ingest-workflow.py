#!/usr/bin/env python3
"""SD-27 workflow driver — dispatch state + reporting for the book pre-build cycles.

This is the bundle's runnable orchestration surface. It does NOT replace the
in-harness `Workflow` tool that actually dispatches cycle bodies (see
`decisions.md §19`, adopting SD-26's `decisions.md §13` finding that a headless
`claude code --profile … --task …` dispatcher does not exist in the live CLI).
What this script owns is the deterministic part the dispatcher must not
improvise: the manifest seed, the claim/complete state machine, the dependency
ordering, and every write to the operator's reporting JSON.

Authored as Python rather than Node so it can import the sanctioned dashboard
writer directly. The alternative — shelling out and marshalling JSON — would
have to re-implement the helper's atomic-write and stat-recompute discipline,
which is exactly what the helper exists to prevent.

Reporting surface
-----------------
All dashboard mutation goes through the orchestrator helper's public API
(`add_manifest_item`, `claim_item`, `complete_item`, …). The helper performs an
atomic tempfile + fsync + os.replace write under an flock, and chmods the result
to 0644 so nginx can serve it. Never write the JSON directly — a hand-edit is
also silently discarded by the next producer tick.

The producer (`pf1e_dashboard_producer.py`) regenerates the dashboard from the
v0.6 markdown every ~5 minutes but *preserves* each manifest's `items` and
`stats`, so writes made here survive a tick. It does NOT preserve a manifest's
`scope` / `workchannel` / `managed_by` — those are reseeded from the producer's
own `_seed_manifests()`, so they cannot be changed from here.

Path portability
----------------
The helper binds its path defaults at import time, so PF1E_JSON_PATH must be set
*before* the module is imported. This script does that in `_load_helper()`;
importing the helper at module scope would defeat it.

Usage
-----
    python3 scripts/book-ingest-workflow.py preflight
    python3 scripts/book-ingest-workflow.py seed [--dry-run]
    python3 scripts/book-ingest-workflow.py status
    python3 scripts/book-ingest-workflow.py next
    python3 scripts/book-ingest-workflow.py claim sd27.advanced_race_guide.pre_build --agent backend
    python3 scripts/book-ingest-workflow.py complete sd27.advanced_race_guide.pre_build --receipt <path>
    python3 scripts/book-ingest-workflow.py block sd27.advanced_race_guide.pre_build --reason "..."

Environment
-----------
    PF1E_JSON_PATH    dashboard JSON (default: the operator's swarm-observer copy)
    PF1E_HELPER_DIR   directory holding the orchestrator helper
    PCGEN_DATA_ROOT   PF1 LST corpus root
"""
from __future__ import annotations

import argparse
import json
import os
import pathlib
import sys

REPO_ROOT = pathlib.Path(__file__).resolve().parent.parent

MANIFEST_ID = "sd27_book_pre_build"
WORKCHANNEL = "SD-27"

DEFAULT_JSON_PATH = "/home/todd/hermes-home/swarm-observer/PF1e-dashboard.json"
DEFAULT_HELPER_DIR = (
    "/home/todd/hermes-home/.hermes/profiles/god-emporer/skills/"
    "release-swarm-observer/scripts"
)
DEFAULT_PCGEN_ROOT = str(
    pathlib.Path.home() / "workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game"
)

BUNDLE_DIR = "docs/release/SD-27-future-state-book-content-ingestion"

# The 2 in-scope books. Operator-pinned to match the dashboard's SD-27
# workchannel ("SD-27 (ARG + PU)"); Adventurer's Guide is SD-30's, not SD-27's.
BOOKS = [
    {
        "id": "advanced_race_guide",
        "title": "Advanced Race Guide",
        "registry_entry": "0003",
    },
    {
        "id": "pathfinder_unchained",
        "title": "Pathfinder Unchained",
        "registry_entry": "0017",
    },
]

# The per-book lifecycle has 4 CONCEPTUAL stages (scope-draft.md §1.2.1), but only
# 3 CYCLES: Stage 1 (license prep) and Stage 2 (pre-build) are explicitly combined
# into a single cycle for both in-scope and future-state books (scope-draft.md §1.2.1
# "Why combine Stages 1+2..."). No cycle anywhere in the bundle claims or completes a
# standalone "license" item — loop-instruction.md §3.3.1/§3.3.3 and epic-breakdown.md
# §2.4 only ever call `claim`/`complete` on `pre_build`, `verify`, `parity`. An earlier
# version of this manifest modeled 4 stages including a standalone "license" item;
# found by independent multi-agent review 2026-07-27 that this leaves a permanently
# unclaimable item (since claim_item enforces depends_on) blocking pre_build forever.
# The manifest below models the 3 cycles that actually exist.
# Order is load-bearing: it becomes the depends_on chain.
STAGES = [
    ("pre_build", "Pre-build — license prep + LST corpus to Shape B v1 JSON cache"),
    ("verify", "Verify — dual-audit gate + schema conformance over the cache"),
    ("parity", "Parity — PCGen parity baseline against the pre-built cache"),
]


# ---------------------------------------------------------------------------
# Helper loading
# ---------------------------------------------------------------------------

def _load_helper():
    """Import the orchestrator helper with its paths pointed at the real JSON.

    The helper binds DEFAULT_JSON_PATH as a function default at `def` time, so
    the environment must be set before the import. Reassigning the module
    constant afterwards does not retarget the already-defined functions.
    """
    json_path = os.environ.setdefault("PF1E_JSON_PATH", DEFAULT_JSON_PATH)
    helper_dir = os.environ.get("PF1E_HELPER_DIR", DEFAULT_HELPER_DIR)

    if not pathlib.Path(helper_dir).is_dir():
        _die(
            f"orchestrator helper directory not found: {helper_dir}\n"
            "Set PF1E_HELPER_DIR to the release-swarm-observer scripts directory."
        )
    if helper_dir not in sys.path:
        sys.path.insert(0, helper_dir)

    try:
        import pf1e_dashboard_producer_orchestrator_helper as helper
    except ImportError as exc:
        _die(f"cannot import the orchestrator helper from {helper_dir}: {exc}")

    # Guard the silent-empty failure mode. The helper's read_json() returns
    # None for a missing/corrupt file, and list_pending_items() turns that into
    # an empty list — so an unreachable dashboard looks exactly like "no work
    # to do". Fail loudly here instead of reporting a successful no-op.
    if helper.read_json() is None:
        _die(
            f"dashboard JSON is unreachable or corrupt at: {json_path}\n"
            "Refusing to continue — an unreachable dashboard is indistinguishable\n"
            "from an empty one through the helper's API, and would be reported as\n"
            "a successful run that wrote nothing. Set PF1E_JSON_PATH correctly."
        )
    return helper


def _die(msg: str, code: int = 2) -> "NoReturn":  # noqa: F821
    print(f"[sd27-workflow] FATAL: {msg}", file=sys.stderr)
    raise SystemExit(code)


def _log(msg: str) -> None:
    print(f"[sd27-workflow] {msg}")


# ---------------------------------------------------------------------------
# Manifest shape
# ---------------------------------------------------------------------------

def item_id(book_id: str, stage: str) -> str:
    return f"sd27.{book_id}.{stage}"


def receipt_path(book_id: str, stage: str) -> str:
    """Per-cycle receipt path. Epic 3 owns parity; Epic 2 owns the rest."""
    epic = "epic_3" if stage == "parity" else "epic_2"
    return f"{BUNDLE_DIR}/artifacts/{epic}/{book_id}_{stage}-cycle_receipt.md"


def build_items() -> list:
    """The 6 manifest items: 2 books x 3 cycles, chained within each book.

    depends_on is intra-book only — the two books are file-disjoint and may run
    in parallel. The chain makes claim_item() refuse an out-of-order claim, so
    stage ordering is enforced by the helper rather than by convention.
    """
    corpus_root = os.environ.get("PCGEN_DATA_ROOT", DEFAULT_PCGEN_ROOT)
    items = []
    for book_index, book in enumerate(BOOKS):
        prior = None
        for stage_index, (stage, stage_scope) in enumerate(STAGES):
            iid = item_id(book["id"], stage)
            items.append(
                {
                    "id": iid,
                    "kind": "book_stage",
                    "scope": f"{book['title']} — {stage_scope}",
                    "status": "pending",
                    "claimed_by": None,
                    "claimed_at": None,
                    "completed_at": None,
                    "output_path": None,
                    "depends_on": [prior] if prior else [],
                    "license": "OGL",
                    "lst_path": f"{corpus_root}/{book['id']}",
                    "lst_sha256": None,
                    # Books interleave (0,10,20,30 / 1,11,21,31) so the two
                    # parallel-safe books surface together on the On Deck panel
                    # rather than one book monopolising the head of the queue.
                    "priority": stage_index * 10 + book_index,
                    "metadata": {
                        "book": book["id"],
                        "book_title": book["title"],
                        "channel": WORKCHANNEL,
                        "stage": stage,
                        "registry_entry": book["registry_entry"],
                        "receipt_path": receipt_path(book["id"], stage),
                        "corpus_path": f"data/corpus/{book['id']}",
                        "stub_path": f"data/stubs/{book['id']}.json",
                    },
                }
            )
            prior = iid
    return items


# ---------------------------------------------------------------------------
# Commands
# ---------------------------------------------------------------------------

def cmd_preflight(args) -> int:
    """Assert the bundle's launch prerequisites. Reports every failure, not just
    the first, so one run tells the operator everything that needs fixing."""
    helper = _load_helper()
    problems = []
    ok = []

    obj = helper.read_json()
    try:
        helper.validate_schema(obj)
        ok.append(f"dashboard schema valid (schema_version={obj['schema_version']})")
    except ValueError as exc:
        problems.append(f"dashboard schema invalid: {exc}")

    manifests = obj.get("manifests", {})
    if MANIFEST_ID in manifests:
        m = manifests[MANIFEST_ID]
        ok.append(
            f"manifest {MANIFEST_ID!r} present "
            f"(workchannel={m['workchannel']}, managed_by={m['managed_by']}, "
            f"items={len(m.get('items', []))})"
        )
    else:
        problems.append(f"manifest {MANIFEST_ID!r} missing from the dashboard")

    channels = {w["id"] for w in obj.get("workchannels", [])}
    if WORKCHANNEL in channels:
        ok.append(f"workchannel {WORKCHANNEL!r} registered")
    else:
        problems.append(f"workchannel {WORKCHANNEL!r} not in the dashboard")

    corpus_root = pathlib.Path(os.environ.get("PCGEN_DATA_ROOT", DEFAULT_PCGEN_ROOT))
    for book in BOOKS:
        src = corpus_root / book["id"]
        lst = sorted(src.glob("*.lst")) if src.is_dir() else []
        if lst:
            ok.append(f"LST corpus for {book['id']}: {len(lst)} files at {src}")
        else:
            problems.append(f"LST corpus missing or empty for {book['id']}: {src}")

        stub = REPO_ROOT / "data/stubs" / f"{book['id']}.json"
        if stub.is_file():
            ok.append(f"stub manifest present: {stub.relative_to(REPO_ROOT)}")
        else:
            problems.append(f"stub manifest missing: {stub}")

    registry = REPO_ROOT / "docs/governance/wired-integration-stubs-registry.md"
    if registry.is_file():
        text = registry.read_text(encoding="utf-8", errors="replace")
        for book in BOOKS:
            if f"### {book['registry_entry']} — " in text:
                ok.append(f"registry entry #{book['registry_entry']} ({book['id']}) present")
            else:
                problems.append(
                    f"registry entry #{book['registry_entry']} ({book['id']}) not found"
                )
    else:
        problems.append(f"stubs registry missing: {registry}")

    for script in (
        "scripts/identifier-discipline-audit.sh",
        "scripts/wired-integration-audit.sh",
    ):
        p = REPO_ROOT / script
        if p.is_file() and os.access(p, os.X_OK):
            ok.append(f"dual-audit gate present: {script}")
        else:
            problems.append(f"dual-audit gate missing or not executable: {script}")

    for line in ok:
        _log(f"OK   {line}")
    for line in problems:
        print(f"[sd27-workflow] FAIL {line}", file=sys.stderr)

    if problems:
        print(
            f"\n[sd27-workflow] preflight FAILED: {len(problems)} problem(s).",
            file=sys.stderr,
        )
        return 1
    _log(f"preflight PASSED: {len(ok)} checks clean.")
    return 0


def cmd_seed(args) -> int:
    """Write the 6 manifest items. Idempotent — add_manifest_item replaces an
    existing item with the same id, so re-seeding does not duplicate rows.
    Refuses to clobber in-flight work unless --force."""
    helper = _load_helper()
    items = build_items()

    existing = {
        it["id"]: it
        for it in helper.read_json()["manifests"][MANIFEST_ID].get("items", [])
    }
    in_flight = [
        i for i in existing.values() if i.get("status") not in (None, "pending")
    ]
    if in_flight and not args.force:
        for it in in_flight:
            print(
                f"[sd27-workflow] FAIL {it['id']} is {it['status']!r}",
                file=sys.stderr,
            )
        _die(
            f"{len(in_flight)} item(s) already past pending; re-seeding would reset "
            "them.\nPass --force only if you intend to discard that progress.",
            code=1,
        )

    if args.dry_run:
        _log(f"dry-run: would write {len(items)} items to {MANIFEST_ID}")
        print(json.dumps(items, indent=2))
        return 0

    for it in items:
        helper.add_manifest_item(MANIFEST_ID, it)
        _log(f"seeded {it['id']} (priority={it['priority']}, deps={it['depends_on']})")

    manifest = helper.read_json()["manifests"][MANIFEST_ID]
    _log(f"manifest now holds {len(manifest['items'])} items; stats={manifest['stats']}")
    return 0


def cmd_status(args) -> int:
    helper = _load_helper()
    manifest = helper.read_json()["manifests"][MANIFEST_ID]
    _log(f"{MANIFEST_ID}: {manifest['scope']}")
    _log(f"stats: {manifest['stats']}")
    if not manifest.get("items"):
        _log("no items — run `seed` first.")
        return 0
    for it in sorted(manifest["items"], key=lambda x: (x.get("priority", 0), x["id"])):
        claimed = f" by {it['claimed_by']}" if it.get("claimed_by") else ""
        out = f" -> {it['output_path']}" if it.get("output_path") else ""
        print(f"  {it['status']:<12} {it['id']}{claimed}{out}")
    return 0


def cmd_next(args) -> int:
    """List actionable items — pending with all dependencies complete."""
    helper = _load_helper()
    pending = helper.list_pending_items(MANIFEST_ID)
    if not pending:
        manifest = helper.read_json()["manifests"][MANIFEST_ID]
        if not manifest.get("items"):
            _log("no items — run `seed` first.")
        else:
            _log("nothing actionable: every item is claimed, complete, or dependency-blocked.")
        return 0
    _log(f"{len(pending)} actionable item(s):")
    for it in pending:
        print(f"  {it['id']}  ({it['scope']})")
        print(f"      receipt: {it['metadata']['receipt_path']}")
    return 0


def cmd_claim(args) -> int:
    helper = _load_helper()
    try:
        it = helper.claim_item(MANIFEST_ID, args.item_id, args.agent)
    except ValueError as exc:
        _die(str(exc), code=1)
    _log(f"claimed {it['id']} by {it['claimed_by']} at {it['claimed_at']}")
    return 0


def cmd_complete(args) -> int:
    helper = _load_helper()
    receipt = args.receipt
    if receipt and not (REPO_ROOT / receipt).is_file():
        # The receipt is the durable proof the cycle ran. Completing without one
        # would record a green item pointing at nothing.
        _die(f"receipt not found: {receipt}\nWrite the cycle receipt before completing.", code=1)
    try:
        it = helper.complete_item(MANIFEST_ID, args.item_id, receipt)
    except ValueError as exc:
        _die(str(exc), code=1)
    _log(f"completed {it['id']} at {it['completed_at']} -> {it['output_path']}")
    manifest = helper.read_json()["manifests"][MANIFEST_ID]
    _log(f"stats: {manifest['stats']}")
    return 0


def cmd_block(args) -> int:
    """Mark an item blocked/failed. Per loop-instruction.md §4 the orchestrator
    does not auto-retry — a failed cycle returns to the operator."""
    helper = _load_helper()
    obj = helper.read_json()
    manifest = obj["manifests"][MANIFEST_ID]
    for it in manifest.get("items", []):
        if it["id"] == args.item_id:
            it["status"] = args.status
            it.setdefault("metadata", {})["block_reason"] = args.reason
            break
    else:
        _die(f"item {args.item_id!r} not found in {MANIFEST_ID}", code=1)
    helper._recompute_stats(manifest)
    helper.write_json(obj)
    _log(f"marked {args.item_id} {args.status}: {args.reason}")
    _log(f"stats: {manifest['stats']}")
    return 0


COMMANDS = {
    "preflight": cmd_preflight,
    "seed": cmd_seed,
    "status": cmd_status,
    "next": cmd_next,
    "claim": cmd_claim,
    "complete": cmd_complete,
    "block": cmd_block,
}


def main() -> int:
    ap = argparse.ArgumentParser(
        description="SD-27 workflow driver — manifest state + dashboard reporting.",
    )
    sub = ap.add_subparsers(dest="command", required=True)

    sub.add_parser("preflight", help="assert launch prerequisites")

    p_seed = sub.add_parser("seed", help="write the 6 manifest items")
    p_seed.add_argument("--dry-run", action="store_true", help="print items, write nothing")
    p_seed.add_argument("--force", action="store_true", help="re-seed over in-flight items")

    sub.add_parser("status", help="print the manifest state")
    sub.add_parser("next", help="list actionable (dependency-satisfied) items")

    p_claim = sub.add_parser("claim", help="pending -> in_progress")
    p_claim.add_argument("item_id")
    p_claim.add_argument("--agent", default="orchestrator")

    p_done = sub.add_parser("complete", help="in_progress -> complete")
    p_done.add_argument("item_id")
    p_done.add_argument("--receipt", required=True, help="repo-relative cycle receipt path")

    p_block = sub.add_parser("block", help="mark an item blocked or failed")
    p_block.add_argument("item_id")
    p_block.add_argument("--reason", required=True)
    p_block.add_argument("--status", default="blocked", choices=["blocked", "failed"])

    args = ap.parse_args()
    return COMMANDS[args.command](args)


if __name__ == "__main__":
    sys.exit(main())
