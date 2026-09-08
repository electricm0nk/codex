#!/usr/bin/env python3
"""Count the PCGen surface on the LIVE side of the repo and fail when it rises
-- `AT-35-E1-005` (`docs/release/SD-35-corpus-sheet-completion/epic-breakdown.md`),
enforcing `decisions.md` §11 and `technical-design.md` §0.

Why this exists
----------------
Operator ruling (SD-35 `decisions.md` §11): PCGen is a converter input and a
test oracle. Nothing on the live side -- the code that computes and prints a
character sheet -- may read a PCGen token, a PCGen formula string, or a
record's `raw_tokens` at run time. SD-31 Decision 20 permitted a run-time
interpreter "for now" to get a build in front of users; nothing since ruled
it back out, and by 2026-09-07 a coarse grep found 78 live-side files naming
the PCGen surface (`content-unit-inventory.md` §6). A permission with no
counter never expires (`decisions.md` §9 L13). This script is the counter.

What it counts
--------------
Under the live roots (`technical-design.md` §0's boundary table)::

    src/rules_core/**   (minus cache_gen/ -- converter code on the wrong side,
                         relocated by AT-35-E6-002 and counted from then on)
    src/saved_character/**
    src/campaign/**
    src/homebrew_authoring/**
    apps/desktop/**     (the Tauri crate AND the frontend)

every source file (`.rs .ts .tsx .js .jsx .mjs .cjs`; never `node_modules/`,
`dist/`, `target/`) is scanned for the PCGen surface:

    identifiers   raw_tokens  PcgenFormulaEvaluator  render_pcgen_desc
                  bonus_stack_reader  pre_tokens
    token syntax  BONUS:  DEFINE:  PRE[A-Z]+:  SAB:  DESC:  %CHOICE  %LIST  TYPE=

A hit is one regex match; a file counts once however many hits it carries.
A mention inside a comment or a doc string counts -- the ruling is "nothing
left of pcgen", and a comment explaining a PCGen token on the live side is
a sign the code next to it still needs one. The five-identifier subset is
reported separately (`identifier_files=` / `identifier_hits=`) because that
is the population the authoring-time "78 files" figure was counting; quote
the gate's line, not 78, from now on.

The tool side -- `src/pcgen_import/**`, `src/bin/**`, `src/oracle_validation/**`,
`scripts/**`, `tests/**` -- is never scanned. It is KEPT for Starfinder
(`decisions.md` §11); a cycle that deletes it is a defect, not a win.

Modes
-----
    --check         live_files=<n> live_hits=<n> baseline_files=<n> baseline_hits=<n> verdict=PASS|FAIL_INCREASED
                    exit 0 on PASS; 1 when EITHER count is above the baseline in
                    scripts/pcgen-residue-baseline.env; 2 when no baseline exists.
    --check --closure
                    live_files=0 live_hits=0 verdict=PASS   (anything else: verdict=FAIL, exit 1)
                    the closure mode, wired from AT-35-E6-004 on.
    --rebaseline    writes scripts/pcgen-residue-baseline.env from the live count --
                    allowed only when no baseline exists yet (the first recording) or
                    when the count went DOWN on both axes and up on neither.
                    Refused otherwise (rebaseline=REFUSED, exit 1): the baseline is a
                    ratchet, never a reset.

`--root <dir>` and `--baseline <file>` exist so the unit tests
(`scripts/tests/test_pcgen_residue_gate.py`) can run the real CLI against a
synthetic tree; the defaults are the repository this file lives in.
"""

import argparse
import datetime as _dt
import os
import re
import subprocess
import sys
from dataclasses import dataclass, field

# --- the boundary (technical-design.md §0) ---------------------------------
# Changing either tuple is a change to the ruling's enforcement surface;
# test_pcgen_residue_gate.py pins both so it cannot happen quietly
# (acceptance-and-verification.md §3a: "a live path quietly added to its
# allow-list is a defect").
LIVE_ROOTS = (
    "src/rules_core",
    "src/saved_character",
    "src/campaign",
    "src/homebrew_authoring",
    "apps/desktop",
)
EXCLUDED_PREFIXES = ("src/rules_core/cache_gen/",)
EXCLUDED_DIR_NAMES = frozenset({"node_modules", "dist", "target", ".git"})
SOURCE_EXTENSIONS = frozenset({".rs", ".ts", ".tsx", ".js", ".jsx", ".mjs", ".cjs"})

# --- the surface (technical-design.md §0) ----------------------------------
IDENTIFIER_PATTERNS = {
    "raw_tokens": r"\braw_tokens\b",
    "PcgenFormulaEvaluator": r"\bPcgenFormulaEvaluator\b",
    "render_pcgen_desc": r"\brender_pcgen_desc\b",
    "bonus_stack_reader": r"\bbonus_stack_reader\b",
    "pre_tokens": r"\bpre_tokens\b",
}
TOKEN_SYNTAX_PATTERNS = {
    "BONUS:": r"\bBONUS:",
    "DEFINE:": r"\bDEFINE:",
    "PRE[A-Z]+:": r"\bPRE[A-Z]+:",
    "SAB:": r"\bSAB:",
    "DESC:": r"\bDESC:",
    "%CHOICE": r"%CHOICE",
    "%LIST": r"%LIST",
    "TYPE=": r"\bTYPE=",
}
PATTERNS = {**IDENTIFIER_PATTERNS, **TOKEN_SYNTAX_PATTERNS}
_COMPILED = {name: re.compile(rx) for name, rx in PATTERNS.items()}

DEFAULT_BASELINE_REL = os.path.join("scripts", "pcgen-residue-baseline.env")
KEY_FILES = "PCGEN_RESIDUE_BASELINE_FILES"
KEY_HITS = "PCGEN_RESIDUE_BASELINE_HITS"
KEY_SHA = "PCGEN_RESIDUE_BASELINE_SHA"
KEY_DATE = "PCGEN_RESIDUE_BASELINE_DATE"


@dataclass
class ScanResult:
    live_files: int = 0
    live_hits: int = 0
    identifier_files: int = 0
    identifier_hits: int = 0
    files: list = field(default_factory=list)
    files_by_pattern: dict = field(default_factory=dict)
    hits_by_pattern: dict = field(default_factory=dict)
    files_by_root: dict = field(default_factory=dict)
    hits_by_root: dict = field(default_factory=dict)


def _iter_live_source_files(root):
    for live_root in LIVE_ROOTS:
        top = os.path.join(root, live_root)
        if not os.path.isdir(top):
            continue
        for dirpath, dirnames, filenames in os.walk(top):
            dirnames[:] = sorted(d for d in dirnames if d not in EXCLUDED_DIR_NAMES)
            for name in sorted(filenames):
                if os.path.splitext(name)[1] not in SOURCE_EXTENSIONS:
                    continue
                abs_path = os.path.join(dirpath, name)
                rel = os.path.relpath(abs_path, root).replace(os.sep, "/")
                if any(rel.startswith(p) for p in EXCLUDED_PREFIXES):
                    continue
                yield live_root, rel, abs_path


def scan(root):
    """Scan the live side under `root`; pure, no baseline involved."""
    res = ScanResult()
    res.files_by_pattern = {n: 0 for n in PATTERNS}
    res.hits_by_pattern = {n: 0 for n in PATTERNS}
    res.files_by_root = {r: 0 for r in LIVE_ROOTS}
    res.hits_by_root = {r: 0 for r in LIVE_ROOTS}
    for live_root, rel, abs_path in _iter_live_source_files(root):
        try:
            with open(abs_path, encoding="utf-8", errors="replace") as fh:
                text = fh.read()
        except OSError:
            continue
        file_hits = 0
        ident_hits = 0
        for name, rx in _COMPILED.items():
            n = len(rx.findall(text))
            if n:
                res.files_by_pattern[name] += 1
                res.hits_by_pattern[name] += n
                file_hits += n
                if name in IDENTIFIER_PATTERNS:
                    ident_hits += n
        if file_hits:
            res.files.append(rel)
            res.live_files += 1
            res.live_hits += file_hits
            res.files_by_root[live_root] += 1
            res.hits_by_root[live_root] += file_hits
        if ident_hits:
            res.identifier_files += 1
            res.identifier_hits += ident_hits
    return res


def read_baseline(path):
    """Return {'files': int, 'hits': int, 'sha': str, 'date': str} or None."""
    if not os.path.isfile(path):
        return None
    values = {}
    with open(path, encoding="utf-8") as fh:
        for line in fh:
            line = line.strip()
            if not line or line.startswith("#") or "=" not in line:
                continue
            key, _, value = line.partition("=")
            values[key.strip()] = value.strip()
    if KEY_FILES not in values or KEY_HITS not in values:
        return None
    return {
        "files": int(values[KEY_FILES]),
        "hits": int(values[KEY_HITS]),
        "sha": values.get(KEY_SHA, "unknown"),
        "date": values.get(KEY_DATE, "unknown"),
    }


def _head_sha(root):
    try:
        out = subprocess.run(
            ["git", "-C", root, "rev-parse", "HEAD"],
            capture_output=True, text=True, check=False, timeout=10,
        )
    except (OSError, subprocess.SubprocessError):
        return "unknown"
    sha = out.stdout.strip()
    return sha if out.returncode == 0 and sha else "unknown"


def write_baseline(path, res, root):
    sha = _head_sha(root)
    today = _dt.date.today().isoformat()
    lines = [
        "# scripts/pcgen-residue-baseline.env -- written by scripts/pcgen_residue_gate.py --rebaseline",
        "#",
        "# The live-side PCGen surface (technical-design.md §0) at the moment it was",
        "# last recorded. `--check` fails when EITHER count is above these values;",
        "# `--rebaseline` rewrites this file only after a cycle that REDUCED the",
        "# count. Never edit by hand: the numbers are a ratchet, not a target.",
        "# Re-derive: python3 scripts/pcgen_residue_gate.py --check",
        "#",
        "# Breakdown at the recording (informational; the gate compares the totals):",
    ]
    for name in PATTERNS:
        lines.append(f"#   pattern {name} files={res.files_by_pattern[name]} hits={res.hits_by_pattern[name]}")
    for live_root in LIVE_ROOTS:
        lines.append(f"#   root {live_root} files={res.files_by_root[live_root]} hits={res.hits_by_root[live_root]}")
    lines.append(f"#   identifier_files={res.identifier_files} identifier_hits={res.identifier_hits}")
    lines += [
        f"{KEY_FILES}={res.live_files}",
        f"{KEY_HITS}={res.live_hits}",
        f"{KEY_SHA}={sha}",
        f"{KEY_DATE}={today}",
        "",
    ]
    os.makedirs(os.path.dirname(os.path.abspath(path)), exist_ok=True)
    with open(path, "w", encoding="utf-8") as fh:
        fh.write("\n".join(lines))


def print_breakdown(res):
    for name in PATTERNS:
        print(f"pattern {name} files={res.files_by_pattern[name]} hits={res.hits_by_pattern[name]}")
    for live_root in LIVE_ROOTS:
        print(f"root {live_root} files={res.files_by_root[live_root]} hits={res.hits_by_root[live_root]}")
    print(f"identifier_files={res.identifier_files} identifier_hits={res.identifier_hits}")


def run_check(root, baseline_path, closure=False, list_files=False):
    res = scan(root)
    print_breakdown(res)
    if list_files:
        for rel in res.files:
            print(f"file {rel}")
    if closure:
        verdict = "PASS" if (res.live_files == 0 and res.live_hits == 0) else "FAIL"
        print(f"live_files={res.live_files} live_hits={res.live_hits} verdict={verdict}")
        return 0 if verdict == "PASS" else 1
    base = read_baseline(baseline_path)
    if base is None:
        print(
            f"live_files={res.live_files} live_hits={res.live_hits} "
            f"baseline_files=- baseline_hits=- verdict=FAIL_NO_BASELINE"
        )
        print(f"no baseline at {baseline_path}; record the first one with --rebaseline", file=sys.stderr)
        return 2
    increased = res.live_files > base["files"] or res.live_hits > base["hits"]
    verdict = "FAIL_INCREASED" if increased else "PASS"
    print(
        f"live_files={res.live_files} live_hits={res.live_hits} "
        f"baseline_files={base['files']} baseline_hits={base['hits']} verdict={verdict}"
    )
    return 1 if increased else 0


def run_rebaseline(root, baseline_path):
    res = scan(root)
    print_breakdown(res)
    base = read_baseline(baseline_path)
    if base is not None:
        went_up = res.live_files > base["files"] or res.live_hits > base["hits"]
        went_down = res.live_files < base["files"] or res.live_hits < base["hits"]
        if went_up or not went_down:
            print(
                f"rebaseline=REFUSED live_files={res.live_files} live_hits={res.live_hits} "
                f"baseline_files={base['files']} baseline_hits={base['hits']} verdict=FAIL_NOT_REDUCED"
            )
            print("--rebaseline is allowed only after a cycle that reduced the count", file=sys.stderr)
            return 1
    write_baseline(baseline_path, res, root)
    prev = "none" if base is None else f"{base['files']}/{base['hits']}"
    print(
        f"rebaseline=RECORDED live_files={res.live_files} live_hits={res.live_hits} "
        f"previous={prev} path={baseline_path}"
    )
    return 0


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--check", action="store_true", help="compare the live count against the baseline")
    parser.add_argument("--closure", action="store_true", help="with --check: pass only at zero")
    parser.add_argument("--rebaseline", action="store_true", help="record the baseline (first time, or after a reduction)")
    parser.add_argument("--list-files", action="store_true", help="with --check: also print every live-side file that hit")
    parser.add_argument("--root", default=None, help="repository root (default: the repo this script lives in)")
    parser.add_argument("--baseline", default=None, help="baseline file (default: <root>/scripts/pcgen-residue-baseline.env)")
    args = parser.parse_args(argv)

    root = os.path.abspath(args.root) if args.root else os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    baseline_path = args.baseline or os.path.join(root, DEFAULT_BASELINE_REL)

    if args.rebaseline and (args.check or args.closure):
        parser.error("--rebaseline cannot be combined with --check/--closure")
    if args.rebaseline:
        return run_rebaseline(root, baseline_path)
    if args.check or args.closure:
        return run_check(root, baseline_path, closure=args.closure, list_files=args.list_files)
    parser.error("one of --check, --check --closure, or --rebaseline is required")
    return 2


if __name__ == "__main__":
    sys.exit(main())
