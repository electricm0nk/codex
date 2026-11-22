#!/usr/bin/env python3
"""The complete census of the live-side PCGen remainder at AT-35-E6-003-FINISH.

Why this exists
---------------
`AT-35-E6-003-FINISH` was dispatched with a floor of **zero live CODE hits**.
This script is the measurement that was taken before any edit, and it answers
three questions the gate alone cannot:

  A. Where are the 304 hits `pcgen_residue_gate.py --check` still reports?
  B. Which of them are executable product code, and which are `#[cfg(test)]`?
  C. **What does the gate not see?**

(C) is the finding. `pcgen_residue_gate.py` counts the identifier
`render_pcgen_desc` with the regex ``\\brender_pcgen_desc\\b``. The live side no
longer calls a function by that name -- it calls `render_pcgen_desc_tokens` and
`render_pcgen_desc_with_values`, whose trailing `_` defeats the word boundary.
The gate therefore prints ``pattern render_pcgen_desc files=0 hits=0`` while
`src/rules_core/` hands PCGen `DESC:` token strings to the PCGen renderer at
run time. The same blind spot covers every other read that moved behind a
`crate::pcgen_import::` function call instead of staying a literal token:
`ingest_record::token_pairs`, `bonus_chain_qualifiers`, `rebuild_bonus_token`,
`lst_parser::*`, `ir_converter::*`, `race_trait_tokens`, `pool_member_tokens`.

This is the `validate-proxies-against-known-truth` shape and `AGENTS.md` rule 7:
the proxy is still making a confident claim (`live_hits=304`, verdict `PASS`)
in a region it was never tested on. Declaring `live_hits=0` under the present
patterns would be a **false closure** for `AT-35-E6-004`.

Run
---
    python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-FINISH_cycle1_residue_census.py

Every number in the cycle-1 receipt is re-derived by this command. Nothing here
mutates the repository, the gate, or the baseline.
"""

import collections
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", ".."))
sys.path.insert(0, os.path.join(ROOT, "scripts"))

import pcgen_residue_gate as G  # noqa: E402

# --- class C: the run-time reads the gate's patterns cannot match -----------
# Each is a `src/pcgen_import/` (converter/tool-side) symbol invoked from a
# LIVE root at run time. None of them is a literal PCGen token, so no
# TOKEN_SYNTAX_PATTERN fires; none of them is spelled exactly like an
# IDENTIFIER_PATTERN, so no identifier fires either.
UNCOUNTED_SYMBOLS = re.compile(
    r"\b(?:"
    r"render_pcgen_desc_tokens|render_pcgen_desc_with_values|desc_token_arguments"
    r"|leaked_pcgen_syntax|PcgenDisplayValues|RenderedPcgenDesc|pcgen_desc"
    r"|ingest_record|ingest_payload|lst_parser|ir_converter|include_resolver"
    r"|race_trait_tokens|pool_member_tokens|bonus_chain_reader|DeclaredBonuses"
    r"|equipment_bonus_reader|source_content_payload|cache_gen"
    r")\b"
)
PCGEN_IMPORT_PATH = re.compile(r"\bpcgen_import\b")


def cfg_test_ranges(lines):
    """0-based inclusive line ranges covered by a `#[cfg(test)]` item."""
    out, i = [], 0
    while i < len(lines):
        if re.match(r"\s*#\[cfg\(test\)\]", lines[i]):
            j, depth, started = i, 0, False
            while j < len(lines):
                depth += lines[j].count("{") - lines[j].count("}")
                if "{" in lines[j]:
                    started = True
                if started and depth <= 0:
                    break
                j += 1
            out.append((i, j))
            i = j + 1
        else:
            i += 1
    return out


def main():
    counted, uncounted = [], []
    for live_root, rel, abs_path in G._iter_live_source_files(ROOT):
        with open(abs_path, encoding="utf-8", errors="replace") as fh:
            lines = fh.read().splitlines()
        ranges = cfg_test_ranges(lines)
        for i, line in enumerate(lines):
            if line.lstrip().startswith("//"):
                continue  # ruling B14: a comment does not execute
            in_test = any(a <= i <= b for a, b in ranges)
            for name, rx in G._COMPILED.items():
                n = len(rx.findall(line))
                if n:
                    counted.append(
                        dict(root=live_root, file=rel, line=i + 1, pattern=name,
                             hits=n, in_cfg_test=in_test, text=line.strip()[:200])
                    )
            if PCGEN_IMPORT_PATH.search(line) or UNCOUNTED_SYMBOLS.search(line):
                uncounted.append(
                    dict(root=live_root, file=rel, line=i + 1,
                         in_cfg_test=in_test, text=line.strip()[:200])
                )

    counted_total = sum(r["hits"] for r in counted)
    a = [r for r in counted if r["in_cfg_test"]]
    b = [r for r in counted if not r["in_cfg_test"]]
    c = [r for r in uncounted if not r["in_cfg_test"]]

    print("=== A+B: hits the gate counts (`pcgen_residue_gate.py --check`) ===")
    print(f"counted_hits={counted_total} counted_files="
          f"{len({r['file'] for r in counted})}")
    by_pat = collections.Counter()
    for r in counted:
        by_pat[r["pattern"]] += r["hits"]
    print("by_pattern=" + ", ".join(f"{k}={v}" for k, v in sorted(by_pat.items())))
    print(f"class_A_in_cfg_test_hits={sum(r['hits'] for r in a)} "
          f"files={len({r['file'] for r in a})}")
    print(f"class_B_executable_hits={sum(r['hits'] for r in b)} "
          f"files={len({r['file'] for r in b})}")
    print()
    print("--- class B, every hit, verbatim (executable product code) ---")
    for r in b:
        print(f"  {r['file']}:{r['line']} [{r['pattern']} x{r['hits']}] {r['text']}")

    print()
    print("=== C: run-time reads of the converter that NO gate pattern matches ===")
    print(f"class_C_lines={len(c)} files={len({r['file'] for r in c})}")
    by_file = collections.Counter(r["file"] for r in c)
    by_root = collections.Counter(r["root"] for r in c)
    print("by_root=" + ", ".join(f"{k}={v}" for k, v in sorted(by_root.items())))
    for f, n in sorted(by_file.items()):
        print(f"  {n:3d}  {f}")
    print()
    print("--- the renderer call sites, which AT-35-E6-003 says must be gone ---")
    renderer = re.compile(r"render_pcgen_desc_(?:tokens|with_values)")
    for r in c:
        if renderer.search(r["text"]):
            print(f"  {r['file']}:{r['line']}  {r['text']}")
    print()
    print("--- proof of the blind spot ---")
    rx = G._COMPILED["render_pcgen_desc"]
    for s in ("render_pcgen_desc", "render_pcgen_desc_tokens",
              "render_pcgen_desc_with_values"):
        print(f"  \\brender_pcgen_desc\\b matches {s!r}: {bool(rx.search(s))}")

    out = os.path.join(HERE, "AT-35-E6-003-FINISH_cycle1_residue_census.json")
    with open(out, "w", encoding="utf-8") as fh:
        json.dump({"class_A_cfg_test": a, "class_B_executable": b,
                   "class_C_uncounted_runtime": c}, fh, indent=1)
    print()
    print(f"census written to {os.path.relpath(out, ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
