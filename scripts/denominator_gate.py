#!/usr/bin/env python3
"""Fail closed on a percentage stated without its denominator in the same
construct -- `AT-33-E1-004` (`docs/release/SD-33-computed-value-verification/
epic-breakdown.md`), enforcing `decisions.md` §2.

Why this exists
----------------
`decisions.md` §2 names three independent occurrences of the identical
defect, in one session, none of them a false number:

    1. `retro.py`'s `deferrals.open` quoted as "10 open, all resolved" --
       actually `deferrals[-limit:]`, the last N. 29 total; 19 never checked.
    2. Gate 2's corpus-wide engine run quoted as "97.9% recognised" -- true
       of the 4,798 units it ran; 41% of the 11,652 that actually exist.
    3. the orchestrator's own scope figure, "8,446 units remaining" -- one
       row of a nine-row cross-tab, quoted as if it were the whole.

**A true number over the wrong denominator is the most expensive error
shape in this program, precisely because the figure is correct and
therefore survives review.** This script is the mechanical control
`workflow-instruction.md` §12 row 2 names -- a check with an exit code, not
a better-worded warning (`decisions.md` §4).

What it checks
--------------
For every target file, line by line: if a line carries a percentage
(a bare `NN%` / `NN.N%` token -- the concrete shape every occurrence above
took), that same line must also carry a denominator marker -- "of <N>",
"out of <N>", an "<N>/<M>" fraction, or the literal word "denominator"
followed by a number. A percentage with no denominator marker anywhere on
its own line is a violation. The unit of "same construct" is the line --
the same granularity `workflow-instruction.md` §6 step 2's identifier/token
audits already use (`git diff --unified=0`, line-addressed).

**The idiom "a false 100%" / "the false-100% shape" is exempted.** This
bundle's own receipts and `progress.md` coined that exact phrase to *name*
the anti-pattern this gate exists to catch -- not to report a measured
percentage. "100%" there names a shape of bad report, and has no
denominator to state because it is not a completion rate over any
population. Only the idiom's own token is blanked before the percent scan
runs; a real, separate percentage placed on the same line is still caught
in full (`test_idiom_does_not_shadow_a_real_percentage_on_the_same_line`).

**Lines inside a fenced code block (``` ... ```) are skipped.** A receipt's
"RED -> GREEN evidence" section verbatim-quotes a malformed fixture's raw
bytes and a real command's raw stdout -- that transcript is evidence *of*
the check firing, not a claim the receipt is itself making, and sanitizing
it to satisfy this gate would misrepresent what the RED case actually
contained. The criterion's own figures live in the receipt schema's
"Figures + their re-derive commands" table (`workflow-instruction.md` §7)
and surrounding prose -- both outside any fence -- which this gate still
checks in full. (Self-referential proof this exclusion is load-bearing,
not a loophole: this script's own receipt, `AT-33-E1-004_cycle_receipt.md`,
quotes its own malformed RED fixture verbatim inside a fence and would
otherwise trip this exact gate on its own evidence section.)

This is a heuristic, not a natural-language parser. It cannot confirm the
denominator it finds is the *correct* one, only that some numeric
denominator was stated in the same construct -- exactly the discipline
`decisions.md` §2's corollary asks for ("read its implementation before
quoting it" starts with "state it next to a number").

Scope
-----
The target set is **this bundle's own** generated evidence --
`docs/release/SD-33-computed-value-verification/artifacts/**/*_cycle_receipt.md`
plus `progress.md` -- **and, as of `AT-33-E1-004`'s scope-widening remediation
(wave 3), this bundle's headline package documents**: `README.md`,
`decisions.md`, `epic-breakdown.md`, `release-notes.md`, `scope-draft.md`,
`kanban.md`, and `THE-BOX.md`. Attempt 3's final-acceptance scan found the
gap this closes: a first probe at the bundle root was never scanned
(`files_checked` stayed at the receipts+progress.md count) because
`DEFAULT_GLOBS` covered only the two original paths -- a percentage stated
without its denominator in one of these seven root documents was invisible
to the gate, which is the least defensible place for that blind spot given
they are the documents an operator actually reads.

Still **not** `receipts.md` (a different thing -- the Epic 6
closure-pipeline's YAML block log, not a place cycle figures are reported).
Still not `technical-design.md`, `risks-and-open-questions.md`,
`acceptance-and-verification.md`, `content-unit-inventory.md`,
`forward-scope-register.md`, or `technical-requirements.md` -- narrower
supporting documents, not the seven the operator reads as the bundle's
own headline account; widening to them is a task of its own, not a side
effect of this remediation. Still not every prior bundle's receipts (261
files repo-wide, unaudited, out of this criterion's scope).

A later bundle extends `DEFAULT_GLOBS` again (or passes its own paths /
sets `DENOMINATOR_GATE_PATHS`, the env var `scripts/verify.sh`'s stage
reads) for its own receipts -- the same per-bundle-hardcoded-path shape
`supersession-gate` already uses for SD-31's register.
"""

import argparse
import glob
import os
import re
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BUNDLE_DIR = os.path.join(
    REPO_ROOT, "docs", "release", "SD-33-computed-value-verification"
)

DEFAULT_GLOBS = [
    os.path.join(BUNDLE_DIR, "artifacts", "**", "*_cycle_receipt.md"),
    os.path.join(BUNDLE_DIR, "progress.md"),
    # AT-33-E1-004 scope-widening remediation (wave 3): the seven headline
    # package documents an operator actually reads, per the module
    # docstring's "Scope" section. Deliberately root-level only (no
    # `artifacts/**` here -- receipts are already covered above).
    os.path.join(BUNDLE_DIR, "README.md"),
    os.path.join(BUNDLE_DIR, "decisions.md"),
    os.path.join(BUNDLE_DIR, "epic-breakdown.md"),
    os.path.join(BUNDLE_DIR, "release-notes.md"),
    os.path.join(BUNDLE_DIR, "scope-draft.md"),
    os.path.join(BUNDLE_DIR, "kanban.md"),
    os.path.join(BUNDLE_DIR, "THE-BOX.md"),
]

# A bare percentage token: digits (commas allowed), optional decimal, a `%`
# immediately after (optionally one space). Matches "41%", "97.9%",
# "**97.9%**"'s inner "97.9%", "4,798%" (unlikely in practice, harmless).
PERCENT_RE = re.compile(r"\d[\d,]*(?:\.\d+)?\s?%")

# A denominator marker anywhere on the same line: "of <number>" (tolerating
# up to 24 chars of markdown/prose in between, e.g. "of the **4,798**"),
# "out of <number>", an explicit "<N>/<M>" fraction, or the literal word
# "denominator" followed by a number within 24 chars.
DENOMINATOR_RE = re.compile(
    r"\bof\b.{0,24}?[\d,]+"
    r"|\bout of\b.{0,24}?[\d,]+"
    r"|[\d,]+\s*/\s*[\d,]+"
    r"|\bdenominator\b.{0,24}?[\d,]+",
    re.IGNORECASE,
)

# The bundle-wide idiom naming the anti-pattern itself -- "a false 100%" /
# "the false-100% shape" -- coined across `progress.md` and the Epic 5
# receipts to name the exact defect `decisions.md` §2 and this gate exist to
# catch (a `complete` claim over a slice, not the population). "100%" in
# that phrase is not a measured figure about any population -- there is no
# denominator to state, because the sentence is not reporting a completion
# rate; it is naming, and disclaiming, the shape of a bad report. Matched
# and blanked out of the line *before* `PERCENT_RE`/`DENOMINATOR_RE` run, so
# only the idiom's own "100%" token is exempted -- a genuine, separate
# percentage claim placed on the same line is still caught in full (proven
# by `test_idiom_does_not_shadow_a_real_percentage_on_the_same_line`).
# AT-33-E6-001's scan misread six of seven original violations as this same
# idiom before the receipts were rewritten; this remediation re-derived the
# live violation set and found the two that survive are both this idiom
# verbatim -- see the remediation cycle receipt for the re-derivation.
FALSE_100_IDIOM_RE = re.compile(r"\bfalse[\s-]100%", re.IGNORECASE)


def find_violations(text, source="<text>"):
    """Return a list of {source, line, text} dicts, one per line that
    carries a percentage with no denominator marker anywhere on that same
    line. Pure function -- no filesystem access -- so it is directly
    unit-testable against synthetic strings, not only real files.

    Lines inside a fenced code block (a line whose stripped form starts
    with ```) are skipped -- see the module docstring's "Lines inside a
    fenced code block" section for why. An odd number of fences (an
    unterminated block running to end-of-file) leaves the remainder of the
    file unchecked -- a documented limitation, not a silent one: malformed
    markdown of that shape is itself a defect worth a human's eye, and this
    gate is heuristic by design (see module docstring)."""
    violations = []
    in_fence = False
    for lineno, line in enumerate(text.splitlines(), start=1):
        if line.strip().startswith("```"):
            in_fence = not in_fence
            continue
        if in_fence:
            continue
        scan_line = FALSE_100_IDIOM_RE.sub(" ", line)
        if PERCENT_RE.search(scan_line) and not DENOMINATOR_RE.search(scan_line):
            violations.append(
                {"source": source, "line": lineno, "text": line.strip()}
            )
    return violations


def expand_paths(patterns):
    """Expand a list of literal paths / glob patterns into a sorted,
    deduplicated list of real files. A pattern with no wildcard characters
    that matches nothing is a hard error (a typo'd explicit path, not a
    legitimately-empty glob) -- returned separately as `missing` so the
    caller can fail closed on it rather than silently checking fewer files
    than asked."""
    expanded = set()
    missing = []
    for pattern in patterns:
        matches = glob.glob(pattern, recursive=True)
        if not matches:
            if any(ch in pattern for ch in "*?["):
                continue
            missing.append(pattern)
            continue
        expanded.update(m for m in matches if os.path.isfile(m))
    return sorted(expanded), missing


def run_check(patterns, out=sys.stdout):
    """Run the gate against `patterns` (or `DEFAULT_GLOBS` if empty),
    printing machine-parseable summary lines (`files_checked=N`,
    `violations=N`) that `scripts/verify.sh`'s stage sed's out, plus one
    `VIOLATION <file>:<line>: <text>` line per hit. Returns the process
    exit code: 0 clean, 1 violations found (or nothing was checked), 2 an
    explicitly-named path does not exist."""
    if not patterns:
        patterns = list(DEFAULT_GLOBS)

    paths, missing = expand_paths(patterns)

    if missing:
        for p in missing:
            print(f"MISSING_PATH: {p}", file=out)
        print("files_checked=0", file=out)
        print("violations=0", file=out)
        return 2

    all_violations = []
    for p in paths:
        with open(p, "r", encoding="utf-8") as f:
            text = f.read()
        all_violations.extend(find_violations(text, source=p))

    for v in all_violations:
        print(f"VIOLATION {v['source']}:{v['line']}: {v['text']}", file=out)

    print(f"files_checked={len(paths)}", file=out)
    print(f"violations={len(all_violations)}", file=out)

    if not paths:
        print("NO_FILES_MATCHED — the gate checked nothing", file=out)
        return 1

    return 0 if not all_violations else 1


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument(
        "--check", action="store_true",
        help="run the denominator check (the only mode this tool has)",
    )
    parser.add_argument(
        "paths", nargs="*",
        help="explicit files/glob patterns to check "
             "(default: this bundle's own cycle receipts + progress.md)",
    )
    args = parser.parse_args(argv)
    return run_check(args.paths)


if __name__ == "__main__":
    sys.exit(main())
