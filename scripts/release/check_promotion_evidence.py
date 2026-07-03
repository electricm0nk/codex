#!/usr/bin/env python3
"""
check_promotion_evidence.py
============================

CI-side evidence validator for the SD-16 / codex-tranche-2.5 promotion
gates. Complements the local helpers in scripts/release/{_lib-gates.sh,
promote-alpha-to-beta.sh, promote-beta-to-stable.sh} (owned by SD16-E6-F1).
E5-F3a owns the **CI gate** that runs at PR-open time on PRs targeting
``test`` or ``main``.

Doctrine mapping (programs/codex/requirements/SD-16-feedback-loop-and-self-
update-hardening/technical-requirements.md section "Promotion Gate Requirements"):

  Beta gate (develop -> test) requires:
    - tranche acceptance criteria pass
    - canonical release notes validate
    - manifest names tranche_id and release_notes_path
    - at least one successful alpha release exists
    - shell consumed alpha update
    - no release-blocking known issues remain
    - PR body includes tranche id, alpha release URL, release notes path,
      verification evidence, known issues summary, alpha-to-beta statement

  Stable gate (test -> main) requires:
    - beta release exists
    - shell can check/consume beta release
    - release notes remain valid
    - no stable blockers remain
    - beta verification passes
    - PR body includes tranche id, beta release URL, release notes path,
      beta verification evidence, known issues summary, beta-to-stable
      statement

Output:
    - JSON evidence summary on stdout (machine-readable for the workflow)
    - GATE=blocked or GATE=ready as the LAST line for cheap CI parsing
    - exit 0 when the gate is ready, exit 1 when the gate is blocked

Invocation:
    check_promotion_evidence.py --lane beta|stable \\
        --pr-body-file <path> | --pr-body-string <text> \\
        [--notes-file <path>] [--manifest-file <path>] \\
        [--release-url <url>] [--verification-evidence <url-or-path>] \\
        [--beta-release-url <url>] [--provenance-url <url>] \\
        [--alpha-tag <tag>] [--beta-tag <tag>] \\
        [--shell-consumed-evidence <url-or-path>]

When invoked with ``--self-test`` the module runs its built-in tests
and exits non-zero if any test fails. The CI workflow calls
``check_promotion_evidence.py --self-test`` to guard the gate surface.

Stdlib only (json, re, sys, urllib.parse, dataclasses, datetime,
argparse, pathlib). No third-party deps. The same restriction applied to
the bash helpers in scripts/release/*.
"""
from __future__ import annotations

import argparse
import dataclasses
import datetime as _dt
import json
import re
import sys
import urllib.parse as _urlparse
from pathlib import Path

# ---------------------------------------------------------------------------
# Constants -- synced with _lib-gates.sh (E6-F1) to avoid drift
# ---------------------------------------------------------------------------

# Canonical repo-relative release-notes path (release notes live in the
# spec-domain artifacts directory; the AV-BR-6 body always records this
# path so reviewers know where to look).
RELEASE_NOTES_REPO_PATH = (
    "programs/codex/requirements/"
    "SD-16-feedback-loop-and-self-update-hardening/release-notes.md"
)

# Tranche identifier (must appear in the manifest AND the PR body).
TRANCHE_ID = "codex-tranche-2-5"

# Required release-notes section headers (doctrine order; release-notes.md
# doctrine forces them in this sequence).
REQUIRED_NOTE_SECTIONS = [
    "## Summary",
    "## User-Visible Changes",
    "## Defects Fixed",
    "## Operational Notes",
    "## Verification Evidence",
    "## Known Issues",
    "## Update Eligibility",
]

# Anti-blocker regex for the beta lane: only [stable-blocker] markers or
# plain [blocker] markers block the stable->main transition. The beta
# lane is more strict: it blocks on plain [blocker] too (the [blocker]
# marker encodes a release-blocking issue for any release channel, and
# [stable-blocker] is a strict-stable-only blocker).
BETA_BLOCKER_REGEX = r"\[(?:stable-)?blocker\]"
STABLE_BLOCKER_REGEX = r"\[stable-blocker\]"

# PR body key blocks (AV-BR-6). Each item is (name, body key header).
PR_BODY_KEYS_BETA = [
    ("tranche_id", "tranche_id:"),
    ("release_notes_path", "release_notes_path:"),
    ("alpha_release_url", "alpha_release_url:"),
    ("verification_evidence", "verification_evidence:"),
    ("known_issues", "known_issues:"),
]

PR_BODY_KEYS_STABLE = [
    ("tranche_id", "tranche_id:"),
    ("release_notes_path", "release_notes_path:"),
    ("beta_release_url", "beta_release_url:"),
    ("provenance_url", "provenance_url:"),
    ("verification_evidence", "verification_evidence:"),
    ("known_issues", "known_issues:"),
]

# Required body statements (channel transition line). These are the
# alpha-to-beta / beta-to-stable accountability lines the doctrine
# demands.
BETA_TRANSITION_PATTERN = r"(?im)^\s*alpha-?to-?beta\s*statement\s*:\s*\S+"
STABLE_TRANSITION_PATTERN = r"(?im)^\s*beta-?to-?stable\s*statement\s*:\s*\S+"


# ---------------------------------------------------------------------------
# Data structures
# ---------------------------------------------------------------------------


@dataclasses.dataclass
class GateCheck:
    """A single gate's outcome."""

    gate: str  # human-readable gate name
    passed: bool
    detail: str  # short evidence-rich detail


@dataclasses.dataclass
class GateReport:
    """Aggregate outcome for one PR's promotion gate evaluation."""

    lane: str  # "beta" or "stable"
    status: str  # "ready" | "blocked"
    checks: list[GateCheck]
    generated_at: str  # ISO-8601 UTC timestamp
    summary: str  # human-readable one-liner

    def to_dict(self) -> dict:
        return {
            "lane": self.lane,
            "status": self.status,
            "generated_at": self.generated_at,
            "summary": self.summary,
            "checks": [
                {"gate": c.gate, "passed": c.passed, "detail": c.detail}
                for c in self.checks
            ],
        }


# ---------------------------------------------------------------------------
# Loaders
# ---------------------------------------------------------------------------


def _load_notes(notes_file: str | None) -> tuple[str | None, list[str]]:
    """Return (notes_text, problems). problems is empty on success."""
    if not notes_file:
        return None, ["release notes file not provided"]
    path = Path(notes_file)
    if not path.is_file():
        return None, [f"release notes file not found: {notes_file}"]
    try:
        return path.read_text(encoding="utf-8"), []
    except OSError as exc:
        return None, [f"unable to read release notes: {exc}"]


def _load_manifest(manifest_file: str | None) -> tuple[dict | None, list[str]]:
    """Return (manifest_dict, problems)."""
    if not manifest_file:
        return None, ["tranche manifest file not provided"]
    path = Path(manifest_file)
    if not path.is_file():
        return None, [f"tranche manifest file not found: {manifest_file}"]
    try:
        text = path.read_text(encoding="utf-8")
    except OSError as exc:
        return None, [f"unable to read manifest: {exc}"]
    try:
        loaded = json.loads(text)
    except json.JSONDecodeError as exc:
        return None, [f"manifest is not valid JSON: {exc}"]
    if not isinstance(loaded, dict):
        return None, ["manifest root must be a JSON object"]
    return loaded, []


def _load_pr_body(
    pr_body_file: str | None, pr_body_string: str | None
) -> tuple[str | None, list[str]]:
    """Return (pr_body_text, problems)."""
    if pr_body_string:
        return pr_body_string, []
    if pr_body_file:
        path = Path(pr_body_file)
        if not path.is_file():
            return None, [f"PR body file not found: {pr_body_file}"]
        try:
            return path.read_text(encoding="utf-8"), []
        except OSError as exc:
            return None, [f"unable to read PR body file: {exc}"]
    return None, [
        "PR body not provided; pass --pr-body-file or --pr-body-string"
    ]


# ---------------------------------------------------------------------------
# Gate evaluations
# ---------------------------------------------------------------------------


def _extract_section(text: str, header: str) -> str:
    """Return the body lines strictly between ``header`` and the next
    ``## `` section header (or EOF). Mirrors ``_lib-gates.sh`` ->
    extract_section()."""
    out: list[str] = []
    in_section = False
    for line in text.splitlines():
        if line.startswith("## "):
            if in_section:
                break
            if line == header:
                in_section = True
                continue
        elif in_section:
            out.append(line)
    return "\n".join(out)


def _check_release_notes(
    notes_text: str | None, blocker_regex: str
) -> GateCheck:
    """Validate canonical release notes: required sections, ordering, no
    channel field; then run the lane-specific blocker regex against the
    Known Issues section."""
    if not notes_text:
        return GateCheck(
            "G_NOTES_VALIDATED", False, "release notes text not provided"
        )

    problems: list[str] = []
    prev_line = 0
    lines = notes_text.splitlines()
    for header in REQUIRED_NOTE_SECTIONS:
        line_no = None
        for idx, line in enumerate(lines, start=1):
            if line == header:
                line_no = idx
                break
        if line_no is None:
            problems.append(f"missing section: {header}")
            continue
        if line_no <= prev_line:
            problems.append(f"section out of order: {header}")
        prev_line = line_no
        body_lines: list[str] = []
        started = False
        for line in lines[line_no:]:
            if started and line.startswith("## "):
                break
            started = True
            body_lines.append(line)
        if not "".join(body_lines).strip():
            problems.append(f"empty section body: {header}")

    if re.search(r"^\s*channel\s*:", notes_text, re.MULTILINE | re.IGNORECASE):
        problems.append(
            "forbidden 'channel:' field present (channel is derived from the "
            "promotion lane, not the notes)"
        )

    if problems:
        return GateCheck(
            "G_NOTES_VALIDATED",
            False,
            "; ".join(problems),
        )

    # Anti-blocker regex against Known Issues section.
    known = _extract_section(notes_text, "## Known Issues")
    flagged = re.findall(blocker_regex, known, flags=re.IGNORECASE)
    if flagged:
        return GateCheck(
            "G_NOTES_VALIDATED",
            False,
            f"Known Issues contains blocker marker(s): {sorted(set(flagged))}",
        )

    return GateCheck(
        "G_NOTES_VALIDATED",
        True,
        "release notes pass doctrine validation (sections, ordering, "
        "no channel field, no blockers)",
    )


def _check_manifest(
    manifest: dict | None, lane: str
) -> GateCheck:
    """The manifest must exist, parse as JSON, name the tranche id, and
    point release_notes_path at the canonical location. Used by both
    beta and stable lanes -- stable inherits the same manifest name
    requirement so the release-notes path never silently drifts."""
    if manifest is None:
        return GateCheck(
            "G_MANIFEST_VALID", False, "manifest not provided / not parseable"
        )
    problems: list[str] = []
    tranche_id = str(manifest.get("tranche_id", "")).strip()
    if not tranche_id:
        problems.append("manifest missing tranche_id")
    elif tranche_id != TRANCHE_ID:
        problems.append(
            f"manifest tranche_id is {tranche_id!r}, expected {TRANCHE_ID!r}"
        )
    release_notes_path = str(manifest.get("release_notes_path", "")).strip()
    if not release_notes_path:
        problems.append("manifest missing release_notes_path")
    elif release_notes_path != RELEASE_NOTES_REPO_PATH:
        problems.append(
            "manifest release_notes_path is "
            f"{release_notes_path!r}, expected {RELEASE_NOTES_REPO_PATH!r}"
        )
    if problems:
        return GateCheck(
            "G_MANIFEST_VALID", False, "; ".join(problems)
        )
    return GateCheck(
        "G_MANIFEST_VALID",
        True,
        f"manifest names tranche_id={TRANCHE_ID} and "
        f"release_notes_path={RELEASE_NOTES_REPO_PATH}",
    )


def _check_pr_body_pr_keys(
    pr_body: str | None,
    pr_body_keys: list[tuple[str, str]],
    lane: str,
) -> GateCheck:
    """All required ``key: value`` lines must be present in the PR body."""
    if not pr_body:
        return GateCheck(
            "G_PR_BODY_KEYS", False, "PR body not available"
        )
    missing: list[str] = []
    for _, header in pr_body_keys:
        if not re.search(rf"(?m)^\s*{re.escape(header)}\s+\S", pr_body):
            missing.append(header.rstrip(":"))
    if missing:
        return GateCheck(
            "G_PR_BODY_KEYS",
            False,
            f"PR body missing keys: {sorted(missing)}",
        )
    return GateCheck(
        "G_PR_BODY_KEYS",
        True,
        f"PR body contains all required {lane} keys: " +
        ", ".join(header.rstrip(":") for _, header in pr_body_keys),
    )


def _check_transition_statement(
    pr_body: str | None, lane: str
) -> GateCheck:
    """PR body must contain the lane-specific transition statement."""
    if not pr_body:
        return GateCheck(
            "G_TRANSITION_STATEMENT", False, "PR body not available"
        )
    if lane == "beta":
        pattern = BETA_TRANSITION_PATTERN
        label = "alpha-to-beta statement"
    elif lane == "stable":
        pattern = STABLE_TRANSITION_PATTERN
        label = "beta-to-stable statement"
    else:
        return GateCheck(
            "G_TRANSITION_STATEMENT", False, f"unknown lane: {lane!r}"
        )
    if re.search(pattern, pr_body):
        return GateCheck(
            "G_TRANSITION_STATEMENT",
            True,
            f"PR body contains {label}",
        )
    return GateCheck(
        "G_TRANSITION_STATEMENT",
        False,
        f"PR body missing required {label} line",
    )


def _check_alpha_release_exists(
    release_url: str | None,
    *,
    alpha_tag: str | None = None,
) -> GateCheck:
    """Beta lane: at least one successful alpha release must exist."""
    if release_url:
        return GateCheck(
            "G_ALPHA_RELEASE_EXISTS",
            True,
            f"alpha_release_url provided: {release_url}",
        )
    if alpha_tag:
        return GateCheck(
            "G_ALPHA_RELEASE_EXISTS",
            True,
            f"alpha tag resolved: {alpha_tag}",
        )
    return GateCheck(
        "G_ALPHA_RELEASE_EXISTS",
        False,
        "no alpha release URL or tag provided; pass --release-url "
        "or --alpha-tag",
    )


def _check_shell_consumed_alpha(evidence: str | None) -> GateCheck:
    """Beta lane: shell must have consumed an alpha update. The
    evidence URL/file must be a non-empty http(s) URL or an existing
    file path (mirror of _lib-gates.sh::is_valid_evidence)."""
    return _check_evidence_present(
        evidence, "G_SHELL_CONSUMED_ALPHA", "shell-consumed alpha evidence"
    )


def _check_beta_release_exists(release_url: str | None) -> GateCheck:
    """Stable lane: the beta release must exist."""
    if release_url:
        return GateCheck(
            "G_BETA_RELEASE_EXISTS",
            True,
            f"beta_release_url provided: {release_url}",
        )
    return GateCheck(
        "G_BETA_RELEASE_EXISTS",
        False,
        "no beta release URL provided; pass --release-url (beta lane "
        "context)",
    )


def _check_beta_verified(evidence: str | None) -> GateCheck:
    """Stable lane: beta verification must pass (URL or existing file)."""
    return _check_evidence_present(
        evidence, "G_BETA_VERIFIED", "beta verification evidence"
    )


def _check_shell_consumed_beta(evidence: str | None) -> GateCheck:
    """Stable lane: shell must have consumed the beta update."""
    return _check_evidence_present(
        evidence, "G_SHELL_CONSUMED_BETA", "shell-consumed beta evidence"
    )


def _check_provenance(provenance_url: str | None) -> GateCheck:
    """Stable lane: provenance file must be reachable (URL parse or
    local file). The local promotion helper actually downloads
    provenance.json; here we just confirm a URL/file was provided and
    is well-formed. The CI workflow re-runs a stricter download as a
    belt-and-braces step."""
    if not provenance_url:
        return GateCheck(
            "G_PROVENANCE_VALID",
            False,
            "provenance_url not provided; pass --provenance-url",
        )
    parsed = _parse_url_or_path(provenance_url)
    if parsed is None:
        return GateCheck(
            "G_PROVENANCE_VALID",
            False,
            f"provenance_url is neither an http(s) URL nor an existing file: "
            f"{provenance_url}",
        )
    return GateCheck(
        "G_PROVENANCE_VALID",
        True,
        f"provenance reference resolves: {parsed}",
    )


def _check_evidence_present(
    evidence: str | None, gate_name: str, label: str
) -> GateCheck:
    if not evidence or not evidence.strip():
        return GateCheck(
            gate_name, False, f"{label} empty or not provided"
        )
    parsed = _parse_url_or_path(evidence)
    if parsed is None:
        return GateCheck(
            gate_name,
            False,
            f"{label} is not a valid http(s) URL or existing file: "
            f"{evidence!r}",
        )
    return GateCheck(gate_name, True, f"{label} resolves: {parsed}")


def _parse_url_or_path(value: str) -> str | None:
    """Return a normalized representation if ``value`` is a usable
    evidence reference, else ``None``. Accepts:
      - http(s) URL with a non-empty host
      - existing file path"""
    if not value:
        return None
    candidate = value.strip()
    if candidate.startswith(("http://", "https://")):
        try:
            parsed = _urlparse.urlparse(candidate)
        except ValueError:
            return None
        if parsed.scheme not in {"http", "https"}:
            return None
        if not parsed.netloc:
            return None
        return candidate
    if Path(candidate).is_file():
        return candidate
    return None


# ---------------------------------------------------------------------------
# Lane orchestration
# ---------------------------------------------------------------------------


def evaluate_lane(
    *,
    lane: str,
    pr_body: str | None,
    notes_text: str | None,
    manifest: dict | None,
    release_url: str | None,
    verification_evidence: str | None,
    shell_consumed_evidence: str | None,
    beta_release_url: str | None,
    provenance_url: str | None,
    alpha_tag: str | None = None,
) -> GateReport:
    """Run the gate checks for one lane and return a structured report.

    ``lane`` is "beta" (develop->test) or "stable" (test->main).
    The beta gates fail when the body lacks the alpha-context fields;
    the stable gates fail when the body lacks the beta-context fields.
    The caller surfaces the report to the workflow via stdout JSON +
    a final ``GATE=ready|blocked`` line + a non-zero exit when blocked.
    """
    if lane == "beta":
        checks = [
            _check_release_notes(notes_text, BETA_BLOCKER_REGEX),
            _check_manifest(manifest, lane),
            _check_pr_body_pr_keys(pr_body, PR_BODY_KEYS_BETA, lane),
            _check_transition_statement(pr_body, lane),
            _check_alpha_release_exists(release_url, alpha_tag=alpha_tag),
            _check_shell_consumed_alpha(shell_consumed_evidence),
        ]
    elif lane == "stable":
        checks = [
            _check_release_notes(notes_text, STABLE_BLOCKER_REGEX),
            _check_manifest(manifest, lane),
            _check_pr_body_pr_keys(pr_body, PR_BODY_KEYS_STABLE, lane),
            _check_transition_statement(pr_body, lane),
            _check_beta_release_exists(release_url),
            _check_beta_verified(verification_evidence),
            _check_shell_consumed_beta(shell_consumed_evidence),
            _check_provenance(provenance_url),
        ]
    else:
        raise ValueError(f"unknown lane: {lane!r}")

    blocked = [c for c in checks if not c.passed]
    status = "ready" if not blocked else "blocked"
    summary = (
        f"all {len(checks)} promotion-gate checks passed; lane={lane}; "
        "ready for merge review"
        if not blocked
        else (
            f"{len(blocked)} of {len(checks)} promotion-gate checks failed; "
            f"lane={lane}; failing gates: "
            + ", ".join(sorted(c.gate for c in blocked))
        )
    )
    return GateReport(
        lane=lane,
        status=status,
        checks=checks,
        generated_at=_dt.datetime.now(tz=_dt.timezone.utc).isoformat(),
        summary=summary,
    )


# ---------------------------------------------------------------------------
# Loader-problem merging
# ---------------------------------------------------------------------------


def _merge_loader_problems(
    report: GateReport,
    *,
    pr_body_problems: list[str],
    notes_problems: list[str],
    manifest_problems: list[str],
) -> GateReport:
    """Replace the corresponding GateCheck entries whose loader failed
    with the loader's concrete detail so the report explains *why*
    each gate was not evaluable."""
    new_checks: list[GateCheck] = []
    for c in report.checks:
        if c.gate == "G_PR_BODY_KEYS" and pr_body_problems:
            new_checks.append(
                GateCheck(
                    gate=c.gate,
                    passed=False,
                    detail="PR body loader failures: "
                    + "; ".join(pr_body_problems),
                )
            )
        elif c.gate == "G_NOTES_VALIDATED" and notes_problems:
            new_checks.append(
                GateCheck(
                    gate=c.gate,
                    passed=False,
                    detail="release-notes loader failures: "
                    + "; ".join(notes_problems),
                )
            )
        elif c.gate == "G_MANIFEST_VALID" and manifest_problems:
            new_checks.append(
                GateCheck(
                    gate=c.gate,
                    passed=False,
                    detail="manifest loader failures: "
                    + "; ".join(manifest_problems),
                )
            )
        else:
            new_checks.append(c)
    blocked = [c for c in new_checks if not c.passed]
    status = "ready" if not blocked else "blocked"
    summary = (
        f"all {len(new_checks)} promotion-gate checks passed; "
        f"lane={report.lane}; ready for merge review"
        if not blocked
        else (
            f"{len(blocked)} of {len(new_checks)} promotion-gate checks "
            f"failed; lane={report.lane}; failing gates: "
            + ", ".join(sorted(c.gate for c in blocked))
        )
    )
    return GateReport(
        lane=report.lane,
        status=status,
        checks=new_checks,
        generated_at=report.generated_at,
        summary=summary,
    )


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------


def _build_arg_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(
        prog="check_promotion_evidence",
        description=(
            "CI-side evidence validator for the SD-16 promotion gates. "
            "Reads the PR body, release notes, tranche manifest, and "
            "release/shell evidence, then emits an evidence-rich JSON "
            "report plus a final STATUS=ready|blocked line."
        ),
    )
    p.add_argument("--lane", choices=("beta", "stable"), required=False)
    p.add_argument("--pr-body-file", default=None)
    p.add_argument("--pr-body-string", default=None)
    p.add_argument("--notes-file", default=None)
    p.add_argument("--manifest-file", default=None)
    p.add_argument("--release-url", default=None)
    p.add_argument(
        "--verification-evidence",
        default=None,
        help=(
            "Either --release-url or --verification-evidence is required "
            "depending on lane; for stable this is the beta "
            "verification URL/file."
        ),
    )
    p.add_argument("--shell-consumed-evidence", default=None)
    p.add_argument("--beta-release-url", default=None)
    p.add_argument("--provenance-url", default=None)
    p.add_argument("--alpha-tag", default=None)
    p.add_argument("--beta-tag", default=None)
    p.add_argument(
        "--self-test",
        action="store_true",
        help="Run the module's built-in tests and exit.",
    )
    p.add_argument(
        "--json-only",
        action="store_true",
        help="Emit only JSON on stdout (no human-readable render).",
    )
    return p


def _render_human(report: GateReport) -> str:
    lines = [
        f"Promotion Gate Check -- lane={report.lane} status={report.status}",
        f"Generated: {report.generated_at}",
        f"Summary: {report.summary}",
        "",
    ]
    for c in report.checks:
        mark = "PASS" if c.passed else "FAIL"
        lines.append(f"  [{mark}] {c.gate}: {c.detail}")
    return "\n".join(lines)


def main(argv: list[str] | None = None) -> int:
    args = _build_arg_parser().parse_args(argv)

    if args.self_test:
        return _run_self_tests()

    if args.lane not in {"beta", "stable"}:
        print("ERROR: --lane must be 'beta' or 'stable'", file=sys.stderr)
        return 2

    pr_body, pr_body_problems = _load_pr_body(
        args.pr_body_file, args.pr_body_string
    )
    notes_text, notes_problems = _load_notes(args.notes_file)
    manifest, manifest_problems = _load_manifest(args.manifest_file)

    # For beta lane, "release exists" maps to either --release-url or
    # --alpha-tag. For stable lane, it maps to --release-url (which the
    # caller wires to the beta release URL via --beta-release-url).
    if args.lane == "beta":
        release_url = args.release_url
    else:
        release_url = args.beta_release_url or args.release_url

    report = evaluate_lane(
        lane=args.lane,
        pr_body=pr_body,
        notes_text=notes_text,
        manifest=manifest,
        release_url=release_url,
        verification_evidence=args.verification_evidence,
        shell_consumed_evidence=args.shell_consumed_evidence,
        beta_release_url=args.beta_release_url,
        provenance_url=args.provenance_url,
        alpha_tag=args.alpha_tag,
    )

    report = _merge_loader_problems(
        report,
        pr_body_problems=pr_body_problems,
        notes_problems=notes_problems,
        manifest_problems=manifest_problems,
    )

    if not args.json_only:
        print(_render_human(report))
        print("")  # blank line before machine section
    print(json.dumps(report.to_dict(), indent=2, sort_keys=True))
    print(f"STATUS={report.status}")
    print(
        "GATE=" + ("ready" if report.status == "ready" else "blocked")
    )
    return 0 if report.status == "ready" else 1


# ---------------------------------------------------------------------------
# Self-tests (built-in RED-GREEN-REFACTOR harness)
# ---------------------------------------------------------------------------


def _t_release_notes_pass() -> None:
    notes = "\n".join(
        [
            "# Title",
            "## Summary",
            "Body.",
            "## User-Visible Changes",
            "- A change.",
            "## Defects Fixed",
            "- A fix.",
            "## Operational Notes",
            "- A note.",
            "## Verification Evidence",
            "- A result.",
            "## Known Issues",
            "- Minor cosmetic flicker.",
            "## Update Eligibility",
            "- AppImage eligible.",
        ]
    )
    check = _check_release_notes(notes, BETA_BLOCKER_REGEX)
    assert check.passed, f"expected pass: {check.detail}"
    assert "doctrine validation" in check.detail


def _t_release_notes_missing_section() -> None:
    notes = "\n".join(
        [
            "# Title",
            "## Summary",
            "Body.",
            "## User-Visible Changes",
            "- A change.",
            "## Defects Fixed",
            "- A fix.",
            "## Operational Notes",
            "- A note.",
            "## Verification Evidence",
            "- A result.",
            "## Known Issues",
            "- Minor.",
            # Update Eligibility intentionally omitted.
        ]
    )
    check = _check_release_notes(notes, BETA_BLOCKER_REGEX)
    assert not check.passed, "expected fail when section missing"


def _t_release_notes_with_blocker_fails_beta() -> None:
    notes = "\n".join(
        [
            "## Summary", "x",
            "## User-Visible Changes", "x",
            "## Defects Fixed", "x",
            "## Operational Notes", "x",
            "## Verification Evidence", "x",
            "## Known Issues",
            "- [Blocker] Crash on first update check.",
            "## Update Eligibility", "x",
        ]
    )
    check = _check_release_notes(notes, BETA_BLOCKER_REGEX)
    assert not check.passed, "expected fail on [Blocker]"
    assert "Blocker" in check.detail or "blocker" in check.detail.lower()


def _t_release_notes_stable_lane_allows_beta_blocker() -> None:
    # Plain "[blocker]" should NOT block the STABLE lane (only
    # [stable-blocker] does). The E6-F1 test harness proves the
    # symmetric property for beta-only [blocker] entries.
    notes = "\n".join(
        [
            "## Summary", "x",
            "## User-Visible Changes", "x",
            "## Defects Fixed", "x",
            "## Operational Notes", "x",
            "## Verification Evidence", "x",
            "## Known Issues",
            "- [blocker] Beta-lane-only issue.",
            "## Update Eligibility", "x",
        ]
    )
    check = _check_release_notes(notes, STABLE_BLOCKER_REGEX)
    assert check.passed, (
        f"expected PASS on plain [blocker] for stable lane; got {check.detail}"
    )


def _t_release_notes_stable_blocker_blocks_stable() -> None:
    notes = "\n".join(
        [
            "## Summary", "x",
            "## User-Visible Changes", "x",
            "## Defects Fixed", "x",
            "## Operational Notes", "x",
            "## Verification Evidence", "x",
            "## Known Issues",
            "- [stable-blocker] Data loss on relaunch.",
            "## Update Eligibility", "x",
        ]
    )
    check = _check_release_notes(notes, STABLE_BLOCKER_REGEX)
    assert not check.passed, "expected fail on [stable-blocker]"
    assert "stable-blocker" in check.detail


def _t_release_notes_rejects_channel_field() -> None:
    notes = "\n".join(
        [
            "## Summary", "x",
            "channel: beta",  # forbidden
            "## User-Visible Changes", "x",
            "## Defects Fixed", "x",
            "## Operational Notes", "x",
            "## Verification Evidence", "x",
            "## Known Issues", "x",
            "## Update Eligibility", "x",
        ]
    )
    check = _check_release_notes(notes, BETA_BLOCKER_REGEX)
    assert not check.passed, "expected fail on forbidden channel field"
    assert "channel" in check.detail.lower()


def _t_manifest_valid() -> None:
    manifest = {
        "tranche_id": TRANCHE_ID,
        "release_notes_path": RELEASE_NOTES_REPO_PATH,
    }
    check = _check_manifest(manifest, "beta")
    assert check.passed, f"expected pass: {check.detail}"


def _t_manifest_wrong_tranche() -> None:
    manifest = {
        "tranche_id": "wrong-tranche",
        "release_notes_path": RELEASE_NOTES_REPO_PATH,
    }
    check = _check_manifest(manifest, "beta")
    assert not check.passed, "expected fail on wrong tranche id"


def _t_manifest_wrong_notes_path() -> None:
    manifest = {
        "tranche_id": TRANCHE_ID,
        "release_notes_path": "elsewhere/notes.md",
    }
    check = _check_manifest(manifest, "beta")
    assert not check.passed, "expected fail on wrong notes path"


def _t_pr_body_beta_keys_present() -> None:
    body = (
        "tranche_id: codex-tranche-2-5\n"
        "release_notes_path: programs/codex/requirements/"
        "SD-16-feedback-loop-and-self-update-hardening/release-notes.md\n"
        "alpha_release_url: https://github.com/x/y/releases/tag/alpha%2F0.0.1\n"
        "verification_evidence: https://ci.example/runs/1\n"
        "known_issues:\n- none\n"
        "alpha-to-beta statement: alpha published, shell consumed; "
        "promoting to beta.\n"
    )
    check = _check_pr_body_pr_keys(body, PR_BODY_KEYS_BETA, "beta")
    assert check.passed, f"expected pass: {check.detail}"


def _t_pr_body_beta_keys_missing_alpha_url() -> None:
    body = (
        "tranche_id: codex-tranche-2-5\n"
        "release_notes_path: programs/codex/requirements/"
        "SD-16-feedback-loop-and-self-update-hardening/release-notes.md\n"
        # alpha_release_url intentionally missing.
        "verification_evidence: https://ci.example/runs/1\n"
        "known_issues:\n- none\n"
    )
    check = _check_pr_body_pr_keys(body, PR_BODY_KEYS_BETA, "beta")
    assert not check.passed
    assert "alpha_release_url" in check.detail


def _t_pr_body_stable_keys_present() -> None:
    body = (
        "tranche_id: codex-tranche-2-5\n"
        "release_notes_path: programs/codex/requirements/"
        "SD-16-feedback-loop-and-self-update-hardening/release-notes.md\n"
        "beta_release_url: https://github.com/x/y/releases/tag/beta%2F0.0.1\n"
        "provenance_url: https://github.com/x/y/releases/download/"
        "beta%2F0.0.1/provenance.json\n"
        "verification_evidence: https://ci.example/runs/1\n"
        "known_issues:\n- none\n"
        "beta-to-stable statement: beta verified, promoting to stable.\n"
    )
    check = _check_pr_body_pr_keys(body, PR_BODY_KEYS_STABLE, "stable")
    assert check.passed, f"expected pass: {check.detail}"


def _t_transition_statement_beta_present() -> None:
    body = "alpha-to-beta statement: promoting.\n"
    check = _check_transition_statement(body, "beta")
    assert check.passed


def _t_transition_statement_beta_missing() -> None:
    body = "no transition line\n"
    check = _check_transition_statement(body, "beta")
    assert not check.passed
    assert "alpha-to-beta" in check.detail


def _t_transition_statement_stable_present() -> None:
    body = "Beta-to-Stable Statement: promoting.\n"
    check = _check_transition_statement(body, "stable")
    assert check.passed


def _t_evidence_url_accept() -> None:
    assert _parse_url_or_path("https://ci.example/runs/1") is not None
    assert _parse_url_or_path("http://ci.example/runs/1") is not None


def _t_evidence_url_rejects_no_host() -> None:
    assert _parse_url_or_path("https://") is None


def _t_evidence_url_rejects_non_http() -> None:
    assert _parse_url_or_path("ftp://ci.example/runs/1") is None


def _t_evidence_file_accept(tmp) -> None:
    from pathlib import Path as _Path
    f = _Path(tmp) / "evidence.txt"
    f.write_text("ok\n")
    assert _parse_url_or_path(str(f)) is not None


def _t_evidence_file_reject_missing(tmp) -> None:
    from pathlib import Path as _Path
    missing = _Path(tmp) / "does-not-exist.log"
    assert _parse_url_or_path(str(missing)) is None


def _t_alpha_release_exists_url() -> None:
    c = _check_alpha_release_exists(
        "https://github.com/x/y/releases/tag/alpha%2F0.0.1"
    )
    assert c.passed


def _t_alpha_release_exists_tag() -> None:
    c = _check_alpha_release_exists(None, alpha_tag="alpha/0.0.1-abc1234")
    assert c.passed


def _t_alpha_release_exists_missing() -> None:
    c = _check_alpha_release_exists(None, alpha_tag=None)
    assert not c.passed


def _t_shell_consumed_alpha_accepts_url() -> None:
    c = _check_shell_consumed_alpha("https://ci.example/runs/1")
    assert c.passed


def _t_shell_consumed_alpha_rejects_empty() -> None:
    c = _check_shell_consumed_alpha("")
    assert not c.passed


def _t_shell_consumed_alpha_rejects_invalid() -> None:
    c = _check_shell_consumed_alpha("this is not a url or file")
    assert not c.passed


def _t_beta_release_missing() -> None:
    c = _check_beta_release_exists(None)
    assert not c.passed


def _t_provenance_accepts_url() -> None:
    c = _check_provenance("https://ci.example/provenance.json")
    assert c.passed


def _t_provenance_rejects_invalid() -> None:
    c = _check_provenance("not a thing")
    assert not c.passed


def _t_lane_evaluation_beta_ready_minimal() -> None:
    notes = "\n".join(
        [
            "## Summary", "summary",
            "## User-Visible Changes", "- x",
            "## Defects Fixed", "- x",
            "## Operational Notes", "- x",
            "## Verification Evidence", "- x",
            "## Known Issues", "- none",
            "## Update Eligibility", "- eligible",
        ]
    )
    manifest = {
        "tranche_id": TRANCHE_ID,
        "release_notes_path": RELEASE_NOTES_REPO_PATH,
    }
    body = (
        "tranche_id: codex-tranche-2-5\n"
        "release_notes_path: programs/codex/requirements/"
        "SD-16-feedback-loop-and-self-update-hardening/release-notes.md\n"
        "alpha_release_url: https://github.com/x/y/releases/tag/"
        "alpha%2F0.0.1\n"
        "verification_evidence: https://ci.example/runs/1\n"
        "known_issues:\n- none\n"
        "alpha-to-beta statement: promoted.\n"
    )
    report = evaluate_lane(
        lane="beta",
        pr_body=body,
        notes_text=notes,
        manifest=manifest,
        release_url=None,
        verification_evidence=None,
        shell_consumed_evidence="https://shell.example/evidence/1",
        beta_release_url=None,
        provenance_url=None,
        alpha_tag="alpha/0.0.1-abc1234",
    )
    assert report.status == "ready", (
        "expected ready; failing gates: "
        f"{[c.gate for c in report.checks if not c.passed]}"
    )


def _t_lane_evaluation_beta_blocked_when_shell_evidence_empty() -> None:
    notes = "\n".join(
        [
            "## Summary", "x",
            "## User-Visible Changes", "- x",
            "## Defects Fixed", "- x",
            "## Operational Notes", "- x",
            "## Verification Evidence", "- x",
            "## Known Issues", "- none",
            "## Update Eligibility", "- eligible",
        ]
    )
    manifest = {
        "tranche_id": TRANCHE_ID,
        "release_notes_path": RELEASE_NOTES_REPO_PATH,
    }
    body = (
        "tranche_id: codex-tranche-2-5\n"
        "release_notes_path: programs/codex/requirements/"
        "SD-16-feedback-loop-and-self-update-hardening/release-notes.md\n"
        "alpha_release_url: https://github.com/x/y/releases/tag/"
        "alpha%2F0.0.1\n"
        "verification_evidence: https://ci.example/runs/1\n"
        "known_issues:\n- none\n"
        "alpha-to-beta statement: promoted.\n"
    )
    report = evaluate_lane(
        lane="beta",
        pr_body=body,
        notes_text=notes,
        manifest=manifest,
        release_url=None,
        verification_evidence=None,
        shell_consumed_evidence="",  # missing evidence
        beta_release_url=None,
        provenance_url=None,
        alpha_tag="alpha/0.0.1-abc1234",
    )
    assert report.status == "blocked"
    failed = {c.gate for c in report.checks if not c.passed}
    assert "G_SHELL_CONSUMED_ALPHA" in failed


def _t_lane_evaluation_stable_ready_minimal() -> None:
    notes = "\n".join(
        [
            "## Summary", "x",
            "## User-Visible Changes", "- x",
            "## Defects Fixed", "- x",
            "## Operational Notes", "- x",
            "## Verification Evidence", "- x",
            "## Known Issues", "- none",
            "## Update Eligibility", "- eligible",
        ]
    )
    manifest = {
        "tranche_id": TRANCHE_ID,
        "release_notes_path": RELEASE_NOTES_REPO_PATH,
    }
    body = (
        "tranche_id: codex-tranche-2-5\n"
        "release_notes_path: programs/codex/requirements/"
        "SD-16-feedback-loop-and-self-update-hardening/release-notes.md\n"
        "beta_release_url: https://github.com/x/y/releases/tag/"
        "beta%2F0.0.1\n"
        "provenance_url: https://github.com/x/y/releases/download/"
        "beta%2F0.0.1/provenance.json\n"
        "verification_evidence: https://ci.example/runs/1\n"
        "known_issues:\n- none\n"
        "beta-to-stable statement: promoted.\n"
    )
    report = evaluate_lane(
        lane="stable",
        pr_body=body,
        notes_text=notes,
        manifest=manifest,
        release_url="https://github.com/x/y/releases/tag/beta%2F0.0.1",
        verification_evidence="https://ci.example/runs/beta",
        shell_consumed_evidence="https://shell.example/evidence/beta",
        beta_release_url="https://github.com/x/y/releases/tag/beta%2F0.0.1",
        provenance_url=(
            "https://github.com/x/y/releases/download/beta%2F0.0.1/"
            "provenance.json"
        ),
    )
    assert report.status == "ready", (
        "expected ready; failing: "
        f"{[c.gate for c in report.checks if not c.passed]}"
    )


def _t_lane_evaluation_stable_blocked_on_stable_blocker() -> None:
    notes = "\n".join(
        [
            "## Summary", "x",
            "## User-Visible Changes", "- x",
            "## Defects Fixed", "- x",
            "## Operational Notes", "- x",
            "## Verification Evidence", "- x",
            "## Known Issues", "- [stable-blocker] Data loss on relaunch.",
            "## Update Eligibility", "- eligible",
        ]
    )
    manifest = {
        "tranche_id": TRANCHE_ID,
        "release_notes_path": RELEASE_NOTES_REPO_PATH,
    }
    body = (
        "tranche_id: codex-tranche-2-5\n"
        "release_notes_path: programs/codex/requirements/"
        "SD-16-feedback-loop-and-self-update-hardening/release-notes.md\n"
        "beta_release_url: https://github.com/x/y/releases/tag/beta%2F0.0.1\n"
        "provenance_url: https://github.com/x/y/releases/download/beta%2F0.0.1/"
        "provenance.json\n"
        "verification_evidence: https://ci.example/runs/1\n"
        "known_issues:\n- [stable-blocker] Data loss on relaunch.\n"
        "beta-to-stable statement: promoting.\n"
    )
    report = evaluate_lane(
        lane="stable",
        pr_body=body,
        notes_text=notes,
        manifest=manifest,
        release_url="https://github.com/x/y/releases/tag/beta%2F0.0.1",
        verification_evidence="https://ci.example/runs/beta",
        shell_consumed_evidence="https://shell.example/evidence/beta",
        beta_release_url="https://github.com/x/y/releases/tag/beta%2F0.0.1",
        provenance_url=(
            "https://github.com/x/y/releases/download/beta%2F0.0.1/"
            "provenance.json"
        ),
    )
    assert report.status == "blocked"
    failed = {c.gate for c in report.checks if not c.passed}
    assert "G_NOTES_VALIDATED" in failed


def _t_merge_loader_problems_replaces_gate() -> None:
    notes_ok = "\n".join(
        [
            "## Summary", "x",
            "## User-Visible Changes", "- x",
            "## Defects Fixed", "- x",
            "## Operational Notes", "- x",
            "## Verification Evidence", "- x",
            "## Known Issues", "- none",
            "## Update Eligibility", "- eligible",
        ]
    )
    manifest_ok = {
        "tranche_id": TRANCHE_ID,
        "release_notes_path": RELEASE_NOTES_REPO_PATH,
    }
    body_ok = (
        "tranche_id: codex-tranche-2-5\n"
        "release_notes_path: programs/codex/requirements/"
        "SD-16-feedback-loop-and-self-update-hardening/release-notes.md\n"
        "alpha_release_url: https://github.com/x/y/releases/tag/"
        "alpha%2F0.0.1\n"
        "verification_evidence: https://ci.example/runs/1\n"
        "known_issues:\n- none\n"
        "alpha-to-beta statement: promoted.\n"
    )
    report = evaluate_lane(
        lane="beta",
        pr_body=body_ok,
        notes_text=notes_ok,
        manifest=manifest_ok,
        release_url=None,
        verification_evidence=None,
        shell_consumed_evidence="https://shell.example/evidence/1",
        beta_release_url=None,
        provenance_url=None,
        alpha_tag="alpha/0.0.1-abc1234",
    )
    merged = _merge_loader_problems(
        report,
        pr_body_problems=["PR body file not found: /nope"],
        notes_problems=[],
        manifest_problems=[],
    )
    pr_check = next(c for c in merged.checks if c.gate == "G_PR_BODY_KEYS")
    assert not pr_check.passed
    assert "loader failures" in pr_check.detail
    assert merged.status == "blocked"


def _run_self_tests() -> int:
    """Built-in RED-GREEN-REFACTOR harness. Returns 0 on green, non-zero
    on red. Tests are deliberately module-private -- they use _t_* names
    so they look like tests rather than public surface."""
    import tempfile

    failures: list[tuple[str, BaseException]] = []
    pure_funcs = [
        _t_release_notes_pass,
        _t_release_notes_missing_section,
        _t_release_notes_with_blocker_fails_beta,
        _t_release_notes_stable_lane_allows_beta_blocker,
        _t_release_notes_stable_blocker_blocks_stable,
        _t_release_notes_rejects_channel_field,
        _t_manifest_valid,
        _t_manifest_wrong_tranche,
        _t_manifest_wrong_notes_path,
        _t_pr_body_beta_keys_present,
        _t_pr_body_beta_keys_missing_alpha_url,
        _t_pr_body_stable_keys_present,
        _t_transition_statement_beta_present,
        _t_transition_statement_beta_missing,
        _t_transition_statement_stable_present,
        _t_evidence_url_accept,
        _t_evidence_url_rejects_no_host,
        _t_evidence_url_rejects_non_http,
        _t_alpha_release_exists_url,
        _t_alpha_release_exists_tag,
        _t_alpha_release_exists_missing,
        _t_shell_consumed_alpha_accepts_url,
        _t_shell_consumed_alpha_rejects_empty,
        _t_shell_consumed_alpha_rejects_invalid,
        _t_beta_release_missing,
        _t_provenance_accepts_url,
        _t_provenance_rejects_invalid,
        _t_lane_evaluation_beta_ready_minimal,
        _t_lane_evaluation_beta_blocked_when_shell_evidence_empty,
        _t_lane_evaluation_stable_ready_minimal,
        _t_lane_evaluation_stable_blocked_on_stable_blocker,
        _t_merge_loader_problems_replaces_gate,
    ]
    with tempfile.TemporaryDirectory() as td:
        for fn in pure_funcs:
            try:
                fn()
            except AssertionError as exc:
                failures.append((fn.__name__, exc))
            except Exception as exc:
                failures.append((fn.__name__, exc))
        # file-based tests take the tmp dir as positional arg
        for fn in (_t_evidence_file_accept, _t_evidence_file_reject_missing):
            try:
                fn(td)
            except AssertionError as exc:
                failures.append((fn.__name__, exc))
            except Exception as exc:
                failures.append((fn.__name__, exc))
    if failures:
        print("SELF-TEST FAILED:", file=sys.stderr)
        for name, exc in failures:
            print(f"  - {name}: {exc}", file=sys.stderr)
        return 1
    print("SELF-TEST PASSED")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
