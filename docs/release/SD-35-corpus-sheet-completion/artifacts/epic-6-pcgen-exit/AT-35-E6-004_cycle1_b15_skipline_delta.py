#!/usr/bin/env python3
"""Re-derive the two B15 figures in AT-35-E6-004_cycle1_receipt.md's figures table.

Both figures compare the residue gate's `cfg_test_ranges` BEFORE ruling B15
landed against the independent per-character scanner in
`AT-35-E6-004_cycle1_independent_residue_census.py`. The receipt originally
carried prose in the command column for both rows ("the census script's first
line, before/after the fix"), which `denominator_gate.py --check-provenance`
correctly refused: a figure whose command cannot be run is not sourced.

This script IS that command. It loads the pre-B15 gate out of git by SHA
(so the "before" number does not depend on the working tree), walks the same
live-root population the gate walks, and prints:

  b15_gate_skipped_lines_before=<N>   the pre-B15 gate's total skipped lines
  b15_gate_skipped_lines_after=<N>    the gate at the working tree
  independent_skipped_lines=<N>       the independent scanner's total
  files_with_cfg_test_item=<N>        the denominator for the two above
  transaction_rs_shipping_lines_hidden_before=<N>
  transaction_rs_total_lines=<N>
  files_hiding_shipping_lines_before=<N>
  total_shipping_lines_hidden_before=<N>

Run from the repo root:

    python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-004_cycle1_b15_skipline_delta.py

`--before-sha` defaults to 96fa840c6b, the parent of 72103a69bc
("AT-35-E6-004 -- the gate reads zero, and it earns it"), which is the commit
that landed B15's `cfg_test_ranges` fix.
"""
import argparse
import importlib.util
import os
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", ".."))

sys.path.insert(0, os.path.join(REPO, "scripts"))
import pcgen_residue_gate as G  # noqa: E402  (the gate AT the working tree)

sys.path.insert(0, HERE)
_census_spec = importlib.util.spec_from_file_location(
    "e6_census",
    os.path.join(HERE, "AT-35-E6-004_cycle1_independent_residue_census.py"),
)


def load_module_from_source(name, source):
    """Import a module from a source string without touching the working tree."""
    with tempfile.TemporaryDirectory() as td:
        path = os.path.join(td, name + ".py")
        with open(path, "w", encoding="utf-8") as fh:
            fh.write(source)
        spec = importlib.util.spec_from_file_location(name, path)
        mod = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(mod)
        return mod


def independent_cfg_ranges_factory():
    """Pull `ind_cfg_ranges` out of the census script without running its main body.

    The census script executes its census at import time, so it is read as text
    and only its two pure helpers (`classify`, `ind_cfg_ranges`) are compiled.
    """
    src = open(_census_spec.origin, encoding="utf-8").read()
    start = src.index("def classify(")
    end = src.index("\nover_skip = []")
    helpers = "import re\nCFG = re.compile(r\"#!?\\[\\s*cfg\\(\\s*test\\s*\\)\\s*\\]\")\n" + src[start:end]
    return load_module_from_source("e6_census_helpers", helpers)


def line_set(ranges):
    out = set()
    for a, b in ranges:
        out.update(range(a, b + 1))
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--before-sha", default="96fa840c6b")
    args = ap.parse_args()

    old_src = subprocess.check_output(
        ["git", "-C", REPO, "show", f"{args.before_sha}:scripts/pcgen_residue_gate.py"],
        text=True,
    )
    old = load_module_from_source("pcgen_residue_gate_before_b15", old_src)
    helpers = independent_cfg_ranges_factory()

    before = after = ind_total = 0
    files_with_cfg = 0
    files_hiding = 0
    total_hidden = 0
    txn_hidden = txn_lines = 0

    for _lr, rel, ap_path in G._iter_live_source_files(REPO):
        text = open(ap_path, encoding="utf-8", errors="replace").read()
        lines = text.splitlines()
        b = line_set(old.cfg_test_ranges(lines))
        a = line_set(G.cfg_test_ranges(lines))
        i = line_set(helpers.ind_cfg_ranges(text))
        before += len(b)
        after += len(a)
        ind_total += len(i)
        if i:
            files_with_cfg += 1
        # Shipping lines the PRE-B15 gate skipped that the independent scanner
        # says are NOT inside a #[cfg(test)] item.
        hidden = b - i
        if hidden:
            files_hiding += 1
            total_hidden += len(hidden)
        if rel.endswith("update/transaction.rs"):
            txn_hidden = len(hidden)
            txn_lines = len(lines)

    print(f"b15_gate_skipped_lines_before={before}")
    print(f"b15_gate_skipped_lines_after={after}")
    print(f"independent_skipped_lines={ind_total}")
    print(f"files_with_cfg_test_item={files_with_cfg}")
    print(f"transaction_rs_shipping_lines_hidden_before={txn_hidden}")
    print(f"transaction_rs_total_lines={txn_lines}")
    print(f"files_hiding_shipping_lines_before={files_hiding}")
    print(f"total_shipping_lines_hidden_before={total_hidden}")


if __name__ == "__main__":
    main()
