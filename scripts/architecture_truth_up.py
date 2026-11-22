#!/usr/bin/env python3
"""
VENDORED 2026-07-27 from the architecture-truth-up skill:
  <hermes>/profiles/god-emporer/skills/devops/architecture-truth-up/
    scripts/architecture_truth_up.py

Vendored verbatim so the SD-27 bundle's cited verification command resolves
repo-locally instead of depending on a machine-local skills tree. Invoke it
through scripts/architecture-truth-up.sh, which supplies the bundle-specific
--bundle/--receipts-md arguments this script requires. Upstream remains the
doctrine owner; re-sync from it rather than editing this copy in place.

Architecture truth-up — pre-PR gate that updates docs/architecture/ to match the diff.

Usage:
    python3 architecture_truth_up.py \
        --integration-target develop \
        --receipts-md docs/release/SD-NN/receipts.md \
        --bundle SD-NN

Pairs with skill `architecture-truth-up`. The script is the load-bearing entry point
for the Epic Closure's pre-PR sub-step. It fires at the gate regardless of whether
the diff has architecture impact — empty updates still write a receipt.

Per operator directive 2026-07-20:
    - Architecture docs describe live current state, not history.
    - Edits REPLACE outdated statements; obsolete content is REMOVED.
    - The script is a sequential gate in the Epic Closure pipeline.
    - Receipts.md append is mandatory even when the diff is empty.
"""

import argparse
import os
import re
import shutil
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path


# Subset of paths whose diff is "architecture-relevant." Anything outside this set
# produces no doc-edit triggers, but the script still runs the verification
# one-liners and writes a receipt.
ARCHITECTURE_RELEVANT_PATH_PATTERNS = [
    re.compile(r"^src/"),
    re.compile(r"^apps/desktop/"),
    re.compile(r"^apps/desktop/src-tauri/"),
    re.compile(r"^schemas/"),
    re.compile(r"^scripts/release/"),
    re.compile(r"^scripts/tranche/"),
    re.compile(r"^tools/release/"),
    re.compile(r"^\.github/workflows/"),
    re.compile(r"^docs/release/"),
    re.compile(r"^docs/architecture/"),
    re.compile(r"^tests/"),
]

# Files explicitly OUT of architecture scope (no edit triggers from changes here,
# but they get the "no architecture impact" receipt if they're the only diff).
NON_ARCHITECTURE_PATTERNS = [
    re.compile(r"^docs/doctrine-external/"),
    re.compile(r"^docs/release/template/"),
]


def log(msg: str) -> None:
    print(f"[truth-up] {msg}")


def err(msg: str) -> None:
    print(f"[truth-up][error] {msg}", file=sys.stderr)


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


def compute_diff_stat(repo_root: Path, target: str) -> tuple[str, list[str], list[str]]:
    """Return (stat_output, changed_paths_under_arch_scope, changed_paths_outside_arch_scope)."""
    try:
        result = subprocess.run(
            ["git", "diff", "--name-only", f"{target}...HEAD"],
            cwd=repo_root,
            capture_output=True,
            text=True,
            check=True,
        )
    except subprocess.CalledProcessError as exc:
        return exc.stderr or "", [], []

    paths = [line.strip() for line in result.stdout.splitlines() if line.strip()]
    in_scope = []
    out_of_scope = []
    for p in paths:
        if any(pat.search(p) for pat in ARCHITECTURE_RELEVANT_PATH_PATTERNS) and not any(
            pat.search(p) for pat in NON_ARCHITECTURE_PATTERNS
        ):
            in_scope.append(p)
        else:
            out_of_scope.append(p)
    return result.stdout, in_scope, out_of_scope


def parse_source_dirs_index(arch_readme: Path) -> list[tuple[str, str]]:
    """Parse the architecture README's index table to get (doc_rel_path, source_dirs) pairs."""
    if not arch_readme.exists():
        return []
    content = arch_readme.read_text(encoding="utf-8")
    rows = []
    # The index table rows look like:
    # || [name.md](./name.md) | Scope | `src/`, `apps/desktop/` ||
    pattern = re.compile(r"\|\|\s*\[([^\]]+\.md)\]\(\./([^)]+)\)\s*\|[^|]+\|\s*`([^`]+)`\s*\|\|")
    for m in pattern.finditer(content):
        rows.append((m.group(2), m.group(3)))  # (relative doc path, source dirs)
    return rows


def map_paths_to_docs(paths: list[str], arch_root: Path) -> dict[str, list[str]]:
    """Return {doc_rel_path: [touched paths]} mapping."""
    arch_readme = arch_root / "README.md"
    rows = parse_source_dirs_index(arch_readme)
    mapping: dict[str, list[str]] = {}
    for p in paths:
        for doc_rel, source_dirs in rows:
            # source_dirs may be a comma-separated list of path globs
            for d in source_dirs.split(","):
                d = d.strip().rstrip("/")
                if d and (p == d or p.startswith(d + "/") or Path(d) == Path(p)):
                    mapping.setdefault(doc_rel, []).append(p)
                    break
    return mapping


def refresh_last_verified(arch_doc: Path, sha: str, when: str) -> bool:
    """Refresh the `Last verified:` header in an architecture doc. Return True if updated."""
    if not arch_doc.exists():
        return False
    content = arch_doc.read_text(encoding="utf-8")
    new_line = f"Last verified: {when[:10]} against {sha}"
    pattern = re.compile(r"^> Last verified:.*$", re.MULTILINE)
    if pattern.search(content):
        new_content = pattern.sub(new_line, content, count=1)
    else:
        # Insert after the > Scope: line if present
        scope_pattern = re.compile(r"^(> Scope:.*\n)", re.MULTILINE)
        if scope_pattern.search(content):
            new_content = scope_pattern.sub(r"\1" + new_line + "\n", content, count=1)
        else:
            # Insert at top after first heading
            h_pattern = re.compile(r"^(# .*\n)", re.MULTILINE)
            if h_pattern.search(content):
                new_content = h_pattern.sub(r"\1\n" + new_line + "\n", content, count=1)
            else:
                return False
    arch_doc.write_text(new_content, encoding="utf-8")
    return True


def run_verification_one_liners(repo_root: Path) -> tuple[bool, bool, list[str]]:
    """Run the cited-path and relative-link checks. Return (cited_pass, link_pass, findings)."""
    findings = []
    cited_cmd = (
        "grep -rhoE '`(src|apps|tests|scripts|tools|schemas|docs|\\.github)/[^`]*`' docs/architecture/*.md "
        "| tr -d '`' | sed 's/[:#].*$//' | grep -vE '[* <]' | sort -u"
    )
    link_cmd = (
        "grep -rhoE '\\]\\(\\./[^)]+\\.md' docs/architecture/*.md "
        "| sed 's/](\\.\\///' | sort -u"
    )
    try:
        cited_proc = subprocess.run(
            cited_cmd, shell=True, cwd=repo_root, capture_output=True, text=True, check=False
        )
        cited_lines = [line.strip() for line in cited_proc.stdout.splitlines() if line.strip()]
        cited_misses = []
        for line in cited_lines:
            if not (repo_root / line).exists():
                cited_misses.append(f"MISSING: {line}")
    except subprocess.SubprocessError as exc:
        cited_misses = [f"cited-path check failed: {exc}"]

    try:
        link_proc = subprocess.run(
            link_cmd, shell=True, cwd=repo_root, capture_output=True, text=True, check=False
        )
        link_lines = [line.strip() for line in link_proc.stdout.splitlines() if line.strip()]
        link_misses = []
        for line in link_lines:
            if not (repo_root / "docs/architecture" / line).is_file():
                link_misses.append(f"BROKEN LINK: {line}")
    except subprocess.SubprocessError as exc:
        link_misses = [f"relative-link check failed: {exc}"]

    findings.extend(cited_misses)
    findings.extend(link_misses)
    return (len(cited_misses) == 0, len(link_misses) == 0, findings)


def append_receipt(
    receipts_path: Path,
    bundle: str,
    branch: str,
    integration_target: str,
    branch_tip_before: str,
    branch_tip_after: str,
    diff_path_count: int,
    docs_touched: list[str],
    stub_graduations: list[str],
    stub_regressions: list[str],
    obsolete_removals: int,
    cited_pass: bool,
    link_pass: bool,
    cycle_id: str,
) -> None:
    """Append a YAML receipt block to the bundle's receipts.md."""
    block = f"""
- cycle_id: {cycle_id}
  row_or_kind: architecture:truth_up
  bundle: {bundle}
  branch: {branch}
  integration_target: {integration_target}
  branch_tip_before: {branch_tip_before}
  branch_tip_after: {branch_tip_after}
  diff_path_count: {diff_path_count}
  docs_touched: [{', '.join(docs_touched) if docs_touched else ''}]
  stub_graduations: [{', '.join(stub_graduations) if stub_graduations else ''}]
  stub_regressions: [{', '.join(stub_regressions) if stub_regressions else ''}]
  obsolete_removals: {obsolete_removals}
  cited_path_check: {'pass' if cited_pass else 'fail'}
  relative_link_check: {'pass' if link_pass else 'fail'}
  evidence_tier_before: (recorded by operator at receipt read time)
  evidence_tier_after: (recorded by operator at receipt read time)
  receipt_note: {'no architecture impact — diff is outside architecture scope' if not docs_touched else f'truth-up touched {len(docs_touched)} doc(s)'}
"""
    with receipts_path.open("a", encoding="utf-8") as f:
        f.write(block)


def main() -> int:
    ap = argparse.ArgumentParser(description="Architecture truth-up — pre-PR gate.")
    ap.add_argument("--integration-target", default="develop", help="Branch to diff against (default: develop)")
    ap.add_argument("--receipts-md", required=True, help="Path to bundle's receipts.md (append-only ledger)")
    ap.add_argument("--bundle", required=True, help="Bundle ID, e.g. SD-NN")
    ap.add_argument("--branch", default="", help="Active tranche branch (read from git if empty)")
    ap.add_argument("--force", action="store_true", help="Allow promotion with dirty working tree")
    ap.add_argument("--dry-run", action="store_true", help="Print what would happen without editing or appending")
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

    arch_root = repo_root / "docs" / "architecture"
    if not arch_root.is_dir():
        err(f"docs/architecture/ not found at {arch_root}")
        return 4
    arch_readme = arch_root / "README.md"
    if not arch_readme.exists():
        err(f"docs/architecture/README.md not found")
        return 5

    # Working-tree cleanliness
    if not args.force:
        porcelain = git_porcelain(repo_root)
        if porcelain.strip():
            err("codex repo working tree is dirty:")
            for line in porcelain.splitlines():
                err(f"  {line}")
            err("refusing to run truth-up. Commit/stash/clean up first, or pass --force.")
            return 6
        log("working tree: clean")
    else:
        log("working tree: skipped (--force)")

    # Compute diff
    branch = args.branch or git_rev_parse(repo_root, "HEAD") or "(detached)"
    branch_short = short_sha(branch)
    log(f"branch: {branch} ({branch_short})")
    log(f"integration target: {args.integration_target}")

    diff_output, in_scope, out_of_scope = compute_diff_stat(repo_root, args.integration_target)
    log(f"diff path count: {len(in_scope) + len(out_of_scope)} (in arch scope: {len(in_scope)}, out of scope: {len(out_of_scope)})")

    # Map to docs
    mapping = map_paths_to_docs(in_scope, arch_root)
    docs_touched = sorted(mapping.keys())
    log(f"docs touched: {docs_touched if docs_touched else 'none — no architecture impact'}")

    # Refresh headers (operator-side; the script refreshes the touched docs' Last verified lines)
    stub_graduations: list[str] = []
    stub_regressions: list[str] = []
    obsolete_removals = 0

    if docs_touched and not args.dry_run:
        for doc_rel in docs_touched:
            doc_path = arch_root / doc_rel
            ok = refresh_last_verified(doc_path, branch_short, now_iso())
            if ok:
                log(f"refreshed Last verified: {doc_rel}")
        # Stub graduation / regression detection is operator-driven; the script
        # cannot detect them automatically without running the diff against status.md
        # table contents. The receipt records what the operator declared.

    # Verification one-liners
    cited_pass, link_pass, findings = run_verification_one_liners(repo_root)
    if findings:
        log("verification findings:")
        for f in findings:
            log(f"  {f}")
    else:
        log("verification: cited-path check + relative-link check both pass")

    # Append receipt (mandatory even on empty diff)
    cycle_id = now_iso()
    if not args.dry_run:
        append_receipt(
            receipts_path=receipts_path,
            bundle=args.bundle,
            branch=branch,
            integration_target=args.integration_target,
            branch_tip_before=branch_short,
            branch_tip_after=branch_short,
            diff_path_count=len(in_scope) + len(out_of_scope),
            docs_touched=docs_touched,
            stub_graduations=stub_graduations,
            stub_regressions=stub_regressions,
            obsolete_removals=obsolete_removals,
            cited_pass=cited_pass,
            link_pass=link_pass,
            cycle_id=cycle_id,
        )
        log(f"appended receipt to {receipts_path}")

    # Hard-stop on verification failure (operator must fix and re-run)
    if not cited_pass or not link_pass:
        err("verification one-liners failed; refusing to mark truth-up complete")
        return 7

    log("architecture truth-up complete")
    return 0


if __name__ == "__main__":
    sys.exit(main())
