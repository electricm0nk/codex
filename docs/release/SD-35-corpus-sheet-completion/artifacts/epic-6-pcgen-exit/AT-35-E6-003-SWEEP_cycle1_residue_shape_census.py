#!/usr/bin/env python3
"""Which of `pcgen_residue_gate.py`'s live files are CODE reads and which are
provenance doc-comments -- AT-35-E6-003-SWEEP, cycle 1.

Why this exists
---------------
Epic 6 has run 22 cycles at ~2.8 live files removed per cycle. The sweep cycle
was dispatched with a 25-file floor and the standing rule that the count must
fall "because the reads are gone", never by lowering the baseline, silencing
the gate, or adding an exclusion list.

This script measures WHAT the remaining live files actually carry, because the
floor and that rule turn out to be in conflict on the current remainder. It
takes `pcgen_residue_gate.py --check --list-files`'s own file list and, for
each file, splits the gate's own regex hits into

  * `comment`  -- the line, left-stripped, starts with `//` (a `//!` module doc
                  or a `///` item doc). Provenance prose: "this number came from
                  `BONUS:SAVE|BASE.Will|classlevel(...)/2+2`".
  * `code`     -- anything else. A real live-side read of the ingest format.

and then reports how many files would still hit the gate after every CODE read
in them were removed. That number is the honest ceiling on what a code-only
cycle can clear.

Run: python3 <this file>    (from the repository root)
"""

import pathlib
import re
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[5]
GATE = ROOT / "scripts" / "pcgen_residue_gate.py"

# The gate's own patterns (scripts/pcgen_residue_gate.py). Kept as one regex so
# a hit here is a hit there; the gate is the authority, this is the shape read.
HIT = re.compile(
    r"raw_tokens|raw_bonus_chains|PcgenFormulaEvaluator|render_pcgen_desc"
    r"|bonus_stack_reader|pre_tokens"
    r"|BONUS:|DEFINE:|PRE[A-Z]+:|SAB:|DESC:|%CHOICE|%LIST|TYPE="
)


def live_files():
    out = subprocess.run(
        [sys.executable, str(GATE), "--check", "--list-files"],
        cwd=ROOT, capture_output=True, text=True,
    ).stdout
    return [l[len("file "):].strip() for l in out.splitlines() if l.startswith("file ")]


def main():
    files = live_files()
    rows = []
    for rel in files:
        comment = code = 0
        for line in (ROOT / rel).read_text().splitlines():
            if not HIT.search(line):
                continue
            if line.lstrip().startswith("//"):
                comment += 1
            else:
                code += 1
        rows.append((rel, comment, code))

    total_files = len(rows)
    comment_only = [r for r in rows if r[2] == 0]
    code_bearing = [r for r in rows if r[2] > 0]
    still_hit_after_code_removal = [r for r in rows if r[1] > 0]

    print(f"live_files={total_files}")
    print(f"comment_hits={sum(r[1] for r in rows)} code_hits={sum(r[2] for r in rows)}")
    print(f"files_comment_only={len(comment_only)}")
    print(f"files_with_code_hits={len(code_bearing)}")
    print(
        "files_still_hitting_after_every_code_read_removed="
        f"{len(still_hit_after_code_removal)}"
    )
    print(
        "max_files_clearable_by_code_work_alone="
        f"{total_files - len(still_hit_after_code_removal)}"
    )
    print()
    print("comment  code  file")
    for rel, comment, code in sorted(rows, key=lambda r: (-r[2], -r[1], r[0])):
        print(f"{comment:7d}{code:6d}  {rel}")


if __name__ == "__main__":
    main()
