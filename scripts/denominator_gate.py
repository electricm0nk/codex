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

# AT-34-E1-006: the default scope widens again to SD-34's own package --
# added alongside SD-33's dir, not in place of it, per this module's own
# docstring ("A later bundle extends `DEFAULT_GLOBS` again ... for its own
# receipts"). SD-34 has no `THE-BOX.md` (`workflow-instruction.md §5`), so
# rather than hardcode a headline-doc list that would hard-error the moment
# a doc is renamed, SD-34's own root is swept with a `*.md` glob -- "every
# SD-34 `.md`" is the criterion's literal bar (`epic-breakdown.md`
# AT-34-E1-006), and a glob naturally includes a doc added after this cycle
# too. `verify.sh --only denominator-gate`'s launch-checklist run at this
# exact glob showed `files_checked=15 violations=0`
# (`workflow-instruction.md §1` item 12) -- the same 15 files this default
# now sweeps without an explicit path.
SD34_BUNDLE_DIR = os.path.join(
    REPO_ROOT, "docs", "release", "SD-34-book-completion"
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
    # AT-34-E1-006: every SD-34 `.md` at the package root, plus its own
    # cycle receipts under `artifacts/**`.
    os.path.join(SD34_BUNDLE_DIR, "artifacts", "**", "*_cycle_receipt.md"),
    os.path.join(SD34_BUNDLE_DIR, "*.md"),
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

# A verbatim-quoted corpus percentile idiom: "NN% chance" names a game
# probability drawn from ingested PF1e rules text (`FRT_HVY`'s "75% chance
# to negate critical hits and sneak attack damage", `ce_feats_...lst`),
# never a completion/coverage figure this gate exists to ground -- the same
# reasoning `FALSE_100_IDIOM_RE` above already applies to a different
# idiom. A percentile game mechanic is always phrased "N% chance (to/of/
# per...)" in PF1e/PCGen source text; a completion or coverage figure in
# this bundle's own prose is never phrased that way (it says "of <N>",
# "N of M", or "DONE"). `AT-34-E6-001` gate lane C (2026-09-01) re-derived
# every live denominator-gate violation in this package and found 9 of 16
# were this exact idiom or prose quoting it (`progress.md` lines 337,
# 1785, 2101, 2634, 2901, 2944, 3001, 3007, 3529) -- a verbatim corpus
# quote is not an ungrounded figure, it is a quotation
# (`AT-34-E6-001`'s own dispatch brief), and rewording game rule text to
# manufacture a fake denominator would misrepresent the corpus. Matched
# and blanked out of the line *before* `PERCENT_RE`/`DENOMINATOR_RE` run,
# so only the idiom's own "NN% chance" token is exempted -- a genuine,
# separate percentage claim placed on the same line is still caught in
# full (proven by
# `test_chance_idiom_does_not_shadow_a_real_percentage_on_the_same_line`).
QUOTED_PROSE_CHANCE_IDIOM_RE = re.compile(
    r"\d[\d,]*(?:\.\d+)?\s?%\s*chance\b", re.IGNORECASE
)

# A short, explicit allowlist of exact verbatim corpus-quote substrings
# this package's own receipts cite directly -- narrower and safer than a
# general "percentage inside quotation marks" heuristic would be (that
# would also exempt a genuine completion claim someone quoted for
# emphasis, which is exactly the failure shape this gate exists to catch).
# Each entry is a real PF1e record's own `DESC:`-token prose, re-verified
# against the live corpus by the cycle that added it here:
#
#   - "Carrying capacity increased by 50%" --
#     `advanced_class_guide:equipment_modifier:burdenless`'s description
#     (`AT-34-E6-001` gate lane C, 2026-09-01; cited in
#     `AT-34-E3-003_u_bucket_render_bug_cycle_receipt.md`).
#
# Matched by exact literal substring (not a regex) and blanked out of the
# line before `PERCENT_RE`/`DENOMINATOR_RE` run, same discipline as the
# two idioms above. A later cycle extends this tuple by appending its own
# corpus-verified quote -- it never widens an existing entry into a
# pattern, and this list is never used to swallow a figure that is not a
# literal, cited corpus quote.
KNOWN_QUOTED_CORPUS_PHRASES = (
    "Carrying capacity increased by 50%",
)


def _blank_known_quoted_corpus_phrases(line):
    for phrase in KNOWN_QUOTED_CORPUS_PHRASES:
        line = line.replace(phrase, " " * len(phrase))
    return line


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
        scan_line = QUOTED_PROSE_CHANCE_IDIOM_RE.sub(" ", scan_line)
        scan_line = _blank_known_quoted_corpus_phrases(scan_line)
        if PERCENT_RE.search(scan_line) and not DENOMINATOR_RE.search(scan_line):
            violations.append(
                {"source": source, "line": lineno, "text": line.strip()}
            )
    return violations


# --- AT-34-E1-006: figure-provenance -------------------------------------
#
# `workflow-instruction.md §12` row 15 (UNENFORCED at SD-34's launch): "a
# vacuous pass is not a pass -- state every gate's population." Closed by
# this stage's own PASS line (see `run_provenance_check`) plus the check
# below: a figure with no re-derive command reachable from it is not a
# figure, it is a recollection (`AGENTS.md` rule 9).
#
# A "figure" here is scoped narrowly and unambiguously, the same
# discipline `PERCENT_RE` uses above: a comma-grouped integer of at least
# four digits (`49,438`, `8,034`) or a bare percentage (`PERCENT_RE`,
# reused). Small bare integers ("8 of 9 tables", "step 2", "attempt 12",
# "§3") are excluded on purpose -- they are indistinguishable from section
# numbers, step counts, and ordinals without natural-language parsing this
# gate does not do, and a narrow gate that never fires a false positive is
# worth more than a wide one nobody trusts (the same tradeoff
# `denominator_gate`'s own docstring makes for percentages).
FIGURE_NUMBER_RE = re.compile(r"\b\d{1,3}(?:,\d{3})+\b")

# A re-derive command "reachable from" the figure: an inline code span
# (single backticks, not a fenced block) on the same line that looks like
# an invocation rather than a bare value. This package's real commands
# span many tools (`python3`, `cargo`, `git`, `jq`, `sed`, `awk`, `wc`,
# `grep`, `bash`, shell pipelines...) -- rather than maintain a whitelist
# that silently excludes whichever tool a receipt happens to use next
# (and risks flagging SD-33's already-committed, out-of-scope receipts as
# violations this cycle cannot fix), a code span counts as a candidate
# command when it contains whitespace: every real command in this package
# takes at least one argument or flag, while a bare value/identifier/path
# citation (`` `26396` ``, `` `magnitude_bearing=26396` ``,
# `` `src/bin/v06_work_inventory.rs:9592-9595` ``) is always a single
# token. This is permissive by design -- it can accept a non-command prose
# phrase quoted in backticks as "reachable" (a false accept), which is the
# safe direction of error for a first-cycle mechanism: it never demands a
# rewrite of an already-correct receipt, only ever catches a line with
# *no* multi-token backtick span at all, or one naming a script that does
# not exist.
INLINE_CODE_RE = re.compile(r"`([^`]+)`")

# A command that names a repo-relative script/binary path: pull out any
# `<word>/<word>...` looking segment ending in a known source extension.
# Used to catch a "wrong-command figure" -- a command citing a script
# that does not exist in this tree at all, which cannot possibly have
# produced the figure it is attached to.
SCRIPT_PATH_RE = re.compile(
    r"\b(?:[\w.-]+/)+[\w.-]+\.(?:py|sh|rs)\b"
)


def _line_has_reachable_command(line, repo_root):
    """True if `line` carries a multi-token inline-code span (a candidate
    command, not a bare value), AND every script/binary path named in
    that command actually resolves under `repo_root`. A command with no
    recognizable script path (e.g. a bare `git log --oneline` or `jq
    '.units | length' docs/work-inventory.json`) is accepted as reachable
    without a filesystem check -- this gate cannot resolve `cargo`/`git`/
    `jq` subcommands to a file, only literal paths. Returns `(reachable,
    why)`; `why` is `None` when reachable, else the unresolved path (a
    "wrong-command figure")."""
    for code in INLINE_CODE_RE.findall(line):
        if not re.search(r"\s", code.strip()):
            continue  # a single token -- a value/identifier/citation, not a command
        script_paths = SCRIPT_PATH_RE.findall(code)
        if not script_paths:
            return True, None
        for sp in script_paths:
            candidate = sp if os.path.isabs(sp) else os.path.join(repo_root, sp)
            if not os.path.isfile(candidate):
                return False, sp
        return True, None
    return False, None


# The obligation this stage enforces lives in one named place per the
# receipt schema (`workflow-instruction.md §7`): the "Figures + their
# re-derive commands" row/section, in either shape a receipt actually
# uses in this package -- a `##` heading (`AT-34-E1-001_cycle_receipt.md`)
# or a top-level `- **Figures + their re-derive commands:**` bullet
# (`AT-34-E1-004_cycle_receipt.md`). Scoping to that section, not the
# whole document, is deliberate: a receipt's "Acceptance criterion" quote,
# "Notes", and "Next-cycle plan" prose *reference* already-sourced figures
# in passing (e.g. "the shape-engine boundary (26,396 magnitude-bearing)")
# without repeating their command on every mention -- flagging those
# would not catch a new defect, it would demand every receipt be rewritten
# to cite its own Figures section on every sentence. The Figures section
# itself is exactly where AGENTS.md rule 9's obligation is discharged or
# is not; that is what this stage exists to prove, and it is what SD-34's
# own committed receipts (`AT-34-E1-001` through `-005`) actually satisfy
# today (`TestProvenanceCleanOnRealReceipts`).
FIGURES_SECTION_START_RE = re.compile(
    r"^(?:#{1,6}\s+|-\s+)?\*{0,2}Figures\b.*re-derive command", re.IGNORECASE
)
# A new top-level receipt field starts the moment either a heading or an
# unindented `- **<Field>:**` bullet appears -- that ends the Figures
# section (exclusive of the boundary line itself).
NEXT_SECTION_RE = re.compile(r"^(?:#{1,6}\s|-\s+\*\*[A-Za-z])")


def _figures_section_line_ranges(text):
    """Return a list of (start_line, end_line) 1-indexed, inclusive
    ranges -- one per 'Figures + their re-derive commands' section found
    in `text`. A file with none returns `[]`."""
    lines = text.splitlines()
    ranges = []
    i = 0
    n = len(lines)
    while i < n:
        if FIGURES_SECTION_START_RE.search(lines[i]):
            start = i + 1  # 1-indexed, this heading/bullet line itself
            j = i + 1
            while j < n and not NEXT_SECTION_RE.search(lines[j]):
                j += 1
            ranges.append((start, j))  # j is 1-indexed exclusive end -> inclusive last content line is j (0-indexed j-1)
            i = j
        else:
            i += 1
    return ranges


def find_provenance_violations(text, source="<text>", repo_root=REPO_ROOT):
    """Return a list of {source, line, text, reason} dicts, one per line
    *inside a 'Figures + their re-derive commands' section* that states a
    figure (`FIGURE_NUMBER_RE` or `PERCENT_RE`) with no re-derive command
    reachable from it on that same line. `reason` is `"unsourced"` (no
    command at all) or `"unresolvable"` (a command names a script path
    that does not exist in this tree -- a wrong-command figure). A file
    with no such section produces no violations -- see the section-scope
    note above `FIGURES_SECTION_START_RE`. Fenced code blocks are skipped,
    identically to `find_violations`."""
    lines = text.splitlines()
    in_scope = [False] * len(lines)
    for start, end in _figures_section_line_ranges(text):
        for lineno in range(start, end + 1):
            if 1 <= lineno <= len(lines):
                in_scope[lineno - 1] = True

    violations = []
    in_fence = False
    for idx, line in enumerate(lines):
        lineno = idx + 1
        if line.strip().startswith("```"):
            in_fence = not in_fence
            continue
        if in_fence or not in_scope[idx]:
            continue
        has_figure = bool(FIGURE_NUMBER_RE.search(line)) or bool(
            PERCENT_RE.search(
                _blank_known_quoted_corpus_phrases(
                    QUOTED_PROSE_CHANCE_IDIOM_RE.sub(" ", FALSE_100_IDIOM_RE.sub(" ", line))
                )
            )
        )
        if not has_figure:
            continue
        reachable, bad_path = _line_has_reachable_command(line, repo_root)
        if reachable:
            continue
        reason = "unresolvable" if bad_path else "unsourced"
        entry = {
            "source": source,
            "line": lineno,
            "text": line.strip(),
            "reason": reason,
        }
        if bad_path:
            entry["bad_path"] = bad_path
        violations.append(entry)
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


# The figure-provenance gate's own default scope is deliberately narrower
# than `DEFAULT_GLOBS`: **this package's own artifacts only**, not
# SD-33's. Two independent reasons, not one: (1) the criterion this stage
# enforces is scoped to "this package" by its own title
# (`AT-34-E1-006 — every figure in this package carries its re-derive
# command`); (2) `workflow-instruction.md`'s own boundary forbids writing
# to `docs/release/SD-33-computed-value-verification/` at all in this
# bundle, so a default that could ever need SD-33 receipts fixed to pass
# would create an unfixable red the moment SD-33's older, differently
# shaped receipts (numbered-list figures, not `- **`/`##`-bounded
# sections) hit this section-boundary heuristic. `DENOMINATOR_GATE_PATHS`
# still overrides this for either stage, same as always.
PROVENANCE_DEFAULT_GLOBS = [
    os.path.join(SD34_BUNDLE_DIR, "artifacts", "**", "*_cycle_receipt.md"),
    os.path.join(SD34_BUNDLE_DIR, "*.md"),
]


def run_provenance_check(patterns, out=sys.stdout, repo_root=REPO_ROOT):
    """Run the figure-provenance gate (AT-34-E1-006) against `patterns`
    (or `PROVENANCE_DEFAULT_GLOBS` if empty -- this package's own
    receipts and root `.md` docs). Prints `files_checked=N`,
    `figures_examined=N` and `violations=N` -- the `figures_examined` line
    is the stage's stated population, closing `workflow-instruction.md
    §12` row 15 ("a vacuous pass is not a pass"). Returns 0 clean, 1 on any
    violation (or nothing checked), 2 on an explicitly-named missing
    path."""
    if not patterns:
        patterns = list(PROVENANCE_DEFAULT_GLOBS)

    paths, missing = expand_paths(patterns)

    if missing:
        for p in missing:
            print(f"MISSING_PATH: {p}", file=out)
        print("files_checked=0", file=out)
        print("figures_examined=0", file=out)
        print("violations=0", file=out)
        return 2

    all_violations = []
    figures_examined = 0
    for p in paths:
        with open(p, "r", encoding="utf-8") as f:
            text = f.read()
        lines = text.splitlines()
        in_scope = [False] * len(lines)
        for start, end in _figures_section_line_ranges(text):
            for lineno in range(start, end + 1):
                if 1 <= lineno <= len(lines):
                    in_scope[lineno - 1] = True
        in_fence = False
        for idx, line in enumerate(lines):
            if line.strip().startswith("```"):
                in_fence = not in_fence
                continue
            if in_fence or not in_scope[idx]:
                continue
            if FIGURE_NUMBER_RE.search(line) or PERCENT_RE.search(
                _blank_known_quoted_corpus_phrases(
                    QUOTED_PROSE_CHANCE_IDIOM_RE.sub(" ", FALSE_100_IDIOM_RE.sub(" ", line))
                )
            ):
                figures_examined += 1
        all_violations.extend(
            find_provenance_violations(text, source=p, repo_root=repo_root)
        )

    for v in all_violations:
        detail = v["reason"]
        if v.get("bad_path"):
            detail += f":{v['bad_path']}"
        print(f"VIOLATION {v['source']}:{v['line']}: [{detail}] {v['text']}", file=out)

    print(f"files_checked={len(paths)}", file=out)
    print(f"figures_examined={figures_examined}", file=out)
    print(f"violations={len(all_violations)}", file=out)

    if not paths:
        print("NO_FILES_MATCHED — the gate checked nothing", file=out)
        return 1

    return 0 if not all_violations else 1


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument(
        "--check", action="store_true",
        help="run the denominator check",
    )
    parser.add_argument(
        "--check-provenance", action="store_true",
        help="run the figure-provenance check (AT-34-E1-006): every "
             "figure carries a re-derive command reachable from it",
    )
    parser.add_argument(
        "paths", nargs="*",
        help="explicit files/glob patterns to check "
             "(default: this bundle's own cycle receipts + progress.md)",
    )
    args = parser.parse_args(argv)
    if args.check_provenance:
        return run_provenance_check(args.paths)
    return run_check(args.paths)


if __name__ == "__main__":
    sys.exit(main())
