#!/usr/bin/env python3
"""How much of the residue gate's remaining count sits inside a `#[cfg(test)]`
module of a live file -- SD-35 Epic 6, AT-35-E6-003-SWEEP cycle 5.

Why this exists
---------------
`scripts/pcgen_residue_gate.py` scans every source file under the five live
roots **whole**. All of `tests/**` is exempt, on the stated ground that it is
test code and not live code (the module docstring says so: "The tool side --
`src/pcgen_import/**`, `src/bin/**`, `src/oracle_validation/**`, `scripts/**`,
`tests/**` -- is never scanned"). But an assertion written in a `#[cfg(test)]
mod tests` INSIDE a live file is the same test code, compiled out of the
shipping library by the same attribute, and it counts.

This is the same shape as operator ruling B14 (`decisions.md` §17), which was
forced by cycle 1's census showing 114 of 197 live files carried nothing but
provenance comments. Cycle 5 measured the equivalent for test modules and
reports it; it does **not** change the gate. Whether a `#[cfg(test)]` module
inside a live file is live code is an operator ruling, not a cycle's call, and
`AT-35-E6-004`'s reachability turns on it.

Output (2026-09-11, at the tree that carries this file)::

    live_hits=764  live_files=68
    hits_inside_cfg_test=360  hits_outside=404
    files_test_only=27  files_with_non_test_hits=41

Run:
    python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-SWEEP_cycle5_test_region_census.py
"""

import collections
import importlib.util
import os
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", ".."))


def load_gate():
    path = os.path.join(REPO, "scripts", "pcgen_residue_gate.py")
    spec = importlib.util.spec_from_file_location("pcgen_residue_gate", path)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def test_regions(lines):
    """1-based line numbers inside a `#[cfg(test)]` item, by brace balance.

    Deliberately simple and deliberately CONSERVATIVE about what it claims: it
    finds the attribute, walks to the opening brace, and closes on balance. A
    brace inside a string literal would fool it, so the number it prints is a
    measurement to be argued from, not a gate.
    """
    inside, i, n = set(), 0, len(lines)
    while i < n:
        if lines[i].strip().startswith("#[cfg(test)]"):
            j = i
            while j < n and "{" not in lines[j]:
                j += 1
            depth, k = 0, j
            while k < n:
                depth += lines[k].count("{") - lines[k].count("}")
                inside.add(k + 1)
                if depth <= 0 and k >= j:
                    break
                k += 1
            i = k + 1
        else:
            i += 1
    return inside


def main():
    gate = load_gate()
    in_test = out_test = 0
    per_test = collections.Counter()
    per_live = collections.Counter()
    for _root, rel, abs_path in gate._iter_live_source_files(REPO):
        lines = open(abs_path, encoding="utf-8", errors="replace").read().splitlines()
        regions = test_regions(lines)
        for idx, line in enumerate(lines, 1):
            if line.lstrip().startswith("//"):
                continue
            hits = sum(len(rx.findall(line)) for rx in gate._COMPILED.values())
            if not hits:
                continue
            if idx in regions:
                in_test += hits
                per_test[rel] += hits
            else:
                out_test += hits
                per_live[rel] += hits
    files = set(per_test) | set(per_live)
    test_only = [f for f in files if per_live[f] == 0]
    print(f"live_hits={in_test + out_test}  live_files={len(files)}")
    print(f"hits_inside_cfg_test={in_test}  hits_outside={out_test}")
    print(f"files_test_only={len(test_only)}  files_with_non_test_hits={len(per_live)}")
    print("\nfiles whose every remaining hit is inside a #[cfg(test)] module:")
    for f in sorted(test_only):
        print(f"  {per_test[f]:5d}  {f}")
    print("\ntop files by NON-test hits (the code-bearing remainder):")
    for f, c in per_live.most_common(15):
        print(f"  {c:5d}  {f}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
