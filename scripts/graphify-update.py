#!/usr/bin/env python3
"""
VENDORED 2026-07-27 from the graphify-update skill:
  <hermes>/profiles/god-emporer/skills/devops/graphify-update/
    scripts/update_graphify.py

Vendored verbatim so the SD-27 bundle's E4.2 closure step resolves repo-locally
instead of depending on a machine-local skills tree. Replaces the bundle's
previously-cited `graphify cluster-only` invocation, which named a binary that
is not on PATH; this script drives that same cluster-only mode and accepts
--graphify-cli / --dry-run. Upstream remains the doctrine owner; re-sync from
it rather than editing this copy in place.

Run graphify against the codex repo and emit a receipt to the bundle's receipts.md.

Usage:
    python3 update_graphify.py \
        --integration-target develop \
        --receipts-md docs/release/SD-NN/receipts.md \
        --bundle SD-NN

Pairs with skill `graphify-update`. The script is the load-bearing entry point
for the Epic Closure pipeline's graphify sub-step. Graphify's exit code is
captured in the receipt; the script does NOT refuse the closure pipeline on
graphify non-zero exit (operator-confirmed 2026-07-20). The script only refuses
on script-level failures (graphify CLI not found, dirty working tree, missing
receipts.md).

Per operator directive 2026-07-20:
    "docs update -> update graphify -> PR open -> fix any merge conflicts -> stop"

Graphify is the documentation-graph generator; prior-session invocations
(2026-07-19) used graphify-minimax against the codex repo with a 500-token
budget, max concurrency of 2, and excluded standard build/output directories.
"""

import argparse
import os
import shutil
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path


DEFAULT_EXCLUDE_DIRS = [
    "node_modules",
    "target",
    "dist",
    "build",
    ".git",
    "out",
    "dist-ssr",
    ".next",
    "coverage",
]

DEFAULT_BUDGET = 500000  # tokens; matches prior-session 500-token * 1000 chunks


def log(msg: str) -> None:
    print(f"[graphify-update] {msg}")


def err(msg: str) -> None:
    print(f"[graphify-update][error] {msg}", file=sys.stderr)


def now_iso() -> str:
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def expand(path: str) -> Path:
    return Path(os.path.expanduser(path)).resolve()


def get_repo_root(start: Path) -> Path | None:
    cur = start
    for _ in range(10):
        if (cur / ".git").exists() or (cur / ".git").is_file():
            return cur
        if cur.parent == cur:
            return None
        cur = cur.parent
    return None


def git_porcelain(repo_root: Path) -> str:
    try:
        result = subprocess.run(
            ["git", "status", "--porcelain"],
            cwd=repo_root,
            capture_output=True,
            text=True,
            check=False,
        )
        return result.stdout
    except FileNotFoundError:
        return "<git not on PATH>"


def short_sha(sha: str) -> str:
    return sha.strip()[:8]


def git_rev_parse(repo_root: Path, ref: str) -> str:
    try:
        result = subprocess.run(
            ["git", "rev-parse", ref],
            cwd=repo_root,
            capture_output=True,
            text=True,
            check=True,
        )
        return result.stdout.strip()
    except subprocess.CalledProcessError:
        return ""


def locate_graphify(override: str | None) -> str | None:
    if override:
        if Path(override).is_file() and os.access(override, os.X_OK):
            return override
        return None
    found = shutil.which("graphify")
    if found:
        return found
    fallback = Path.home() / ".local" / "bin" / "graphify"
    if fallback.is_file() and os.access(fallback, os.X_OK):
        return str(fallback)
    return None


def append_receipt(
    receipts_path: Path,
    bundle: str,
    branch: str,
    integration_target: str,
    branch_tip: str,
    graphify_exit_code: int,
    outcome: str,
    wall_clock_seconds: float,
    log_path: str,
    cycle_id: str,
) -> None:
    block = f"""
- cycle_id: {cycle_id}
  row_or_kind: graphify:update
  bundle: {bundle}
  branch: {branch}
  integration_target: {integration_target}
  branch_tip: {branch_tip}
  graphify_exit_code: {graphify_exit_code}
  outcome: {outcome}
  wall_clock_seconds: {wall_clock_seconds:.1f}
  log_path: {log_path}
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: {'graphify succeeded' if outcome == 'success' else f'graphify exited {graphify_exit_code}; operator to decide retry-vs-proceed (see log)'}
"""
    with receipts_path.open("a", encoding="utf-8") as f:
        f.write(block)


def main() -> int:
    ap = argparse.ArgumentParser(description="Run graphify and emit a receipt.")
    ap.add_argument("--integration-target", default="develop", help="Branch to capture branch_tip from (default: develop)")
    ap.add_argument("--receipts-md", required=True, help="Path to bundle's receipts.md (append-only ledger)")
    ap.add_argument("--bundle", required=True, help="Bundle ID, e.g. SD-NN")
    ap.add_argument("--branch", default="", help="Active tranche branch (read from git if empty)")
    ap.add_argument("--graphify-cli", default="", help="Override path to graphify CLI")
    ap.add_argument("--budget", type=int, default=DEFAULT_BUDGET, help="Token budget (default: 500000)")
    ap.add_argument("--exclude", default=",".join(DEFAULT_EXCLUDE_DIRS), help="Comma-separated exclude dirs")
    ap.add_argument("--graphify-out", default="graphify-out", help="Output directory for raw graphify logs")
    ap.add_argument("--force", action="store_true", help="Allow promotion with dirty working tree")
    ap.add_argument("--dry-run", action="store_true", help="Print what would happen without running graphify")
    args = ap.parse_args()

    receipts_path = expand(args.receipts_md)
    if not receipts_path.exists():
        err(f"receipts.md does not exist: {receipts_path}")
        err("the bundle must have a receipts.md before this script fires")
        return 2
    log(f"receipts.md: {receipts_path}")

    bundle_root = receipts_path.parent
    repo_root = get_repo_root(bundle_root)
    if repo_root is None:
        err(f"could not locate .git/ above {bundle_root}")
        return 3
    log(f"repo root: {repo_root}")

    # Working-tree cleanliness
    if not args.force:
        porcelain = git_porcelain(repo_root)
        if porcelain.strip():
            err("codex repo working tree is dirty:")
            for line in porcelain.splitlines():
                err(f"  {line}")
            err("refusing to run graphify. Commit/stash/clean up first, or pass --force.")
            return 4
        log("working tree: clean")
    else:
        log("working tree: skipped (--force)")

    branch = args.branch or git_rev_parse(repo_root, "HEAD") or "(detached)"
    branch_short = short_sha(branch)
    log(f"branch: {branch} ({branch_short})")
    log(f"integration target: {args.integration_target}")

    # Locate graphify
    graphify_cli = locate_graphify(args.graphify_cli or None)
    if graphify_cli is None:
        cycle_id = now_iso()
        log(f"graphify CLI not found; refusing script-level failure")
        if not args.dry_run:
            append_receipt(
                receipts_path=receipts_path,
                bundle=args.bundle,
                branch=branch,
                integration_target=args.integration_target,
                branch_tip=branch_short,
                graphify_exit_code=-1,
                outcome="failed",
                wall_clock_seconds=0.0,
                log_path="<not written — graphify CLI not found>",
                cycle_id=cycle_id,
            )
        err("graphify CLI not found on PATH or at ~/.local/bin/graphify")
        err("install graphify or pass --graphify-cli <path>")
        return 5
    log(f"graphify CLI: {graphify_cli}")

    # Build invocation
    cmd = [
        graphify_cli,
        "cluster-only",
        str(repo_root),
        "--budget", str(args.budget),
        "--exclude", args.exclude,
    ]
    log(f"invocation: {' '.join(cmd)}")

    cycle_id = now_iso()
    log_path = repo_root / args.graphify_out / f".truth-up-run-{cycle_id}.log"
    log_path.parent.mkdir(parents=True, exist_ok=True)

    if args.dry_run:
        log("dry-run; not invoking graphify")
        return 0

    # Invoke graphify
    start = time.monotonic()
    try:
        proc = subprocess.run(
            cmd,
            cwd=repo_root,
            capture_output=True,
            text=True,
            check=False,
            timeout=7200,  # 2-hour hard cap; graphify on the codex repo is ~14 min at 500-token budget
        )
        elapsed = time.monotonic() - start
        exit_code = proc.returncode
        outcome = "success" if exit_code == 0 else "failed"
        log(f"graphify exit={exit_code}, elapsed={elapsed:.1f}s, outcome={outcome}")

        # Write log file (stdout + stderr)
        log_path.write_text(
            f"# graphify run {cycle_id}\n# cmd: {' '.join(cmd)}\n# exit: {exit_code}\n\n"
            f"--- stdout ---\n{proc.stdout}\n--- stderr ---\n{proc.stderr}\n",
            encoding="utf-8",
        )
        log(f"log written: {log_path.relative_to(repo_root)}")

    except subprocess.TimeoutExpired as exc:
        elapsed = time.monotonic() - start
        exit_code = -1
        outcome = "failed"
        err(f"graphify timed out after {elapsed:.1f}s")
        log_path.write_text(
            f"# graphify run {cycle_id} — TIMEOUT\n# cmd: {' '.join(cmd)}\n# stderr: {exc.stderr or ''}\n",
            encoding="utf-8",
        )

    except FileNotFoundError as exc:
        elapsed = time.monotonic() - start
        exit_code = -1
        outcome = "failed"
        err(f"graphify CLI disappeared mid-run: {exc}")

    # Emit receipt (success OR failure — operator decides retry-vs-proceed)
    append_receipt(
        receipts_path=receipts_path,
        bundle=args.bundle,
        branch=branch,
        integration_target=args.integration_target,
        branch_tip=branch_short,
        graphify_exit_code=exit_code,
        outcome=outcome,
        wall_clock_seconds=elapsed,
        log_path=str(log_path.relative_to(repo_root)),
        cycle_id=cycle_id,
    )
    log(f"appended receipt to {receipts_path}")

    if outcome == "success":
        log("graphify update complete")
        return 0
    log("graphify update complete (with failure receipt; pipeline continues per operator directive)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
