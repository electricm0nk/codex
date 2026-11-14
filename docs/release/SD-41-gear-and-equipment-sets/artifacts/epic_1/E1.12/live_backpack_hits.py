#!/usr/bin/env python3
"""SD-41 E1.12: lines naming "backpack" in live Rust code under src/ and apps/desktop/src-tauri/src/.

Live = outside `#[cfg(test)]` items and not a `//` comment line. Prints the count, then each hit.
The package's first command (`grep -rn -i backpack ... | grep -v test`) also counts test-module and
doc-comment lines: it printed 25 on the base, before E1.12 touched anything (decisions.md §20).
Run from the repo root.
"""
import pathlib, subprocess

files = subprocess.check_output(["git", "ls-files", "src", "apps/desktop/src-tauri/src"]).decode().split()
hits = []
for f in files:
    if not f.endswith(".rs"):
        continue
    in_test, pending, bal = False, False, 0
    for i, line in enumerate(pathlib.Path(f).read_text().splitlines(), 1):
        s = line.strip()
        if not in_test and s.startswith("#[cfg(test)]"):
            pending = True
            continue
        if pending:
            if "{" in line:
                bal, pending = line.count("{") - line.count("}"), False
                in_test = bal > 0
            continue
        if in_test:
            bal += line.count("{") - line.count("}")
            in_test = bal > 0
            continue
        if "backpack" in line.lower() and not s.startswith("//"):
            hits.append(f"{f}:{i}:{s[:120]}")
print(len(hits))
print("\n".join(hits))
