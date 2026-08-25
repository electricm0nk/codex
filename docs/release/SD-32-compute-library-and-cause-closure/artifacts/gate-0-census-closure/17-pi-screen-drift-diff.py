#!/usr/bin/env python3
"""decisions.md §17 item 2 -- diff `pi_screen`/`min_level` across the seven
per-book spell-ingest binaries that `src/bin/ingest_spells.rs` collapsed.

The seven binaries are deleted on the branch that carries this collapse
(`git log --oneline -- src/bin/ingest_adventurers_guide_spells.rs` finds the
deletion commit). Re-derive against the last commit that still has them:

    python3 docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/17-pi-screen-drift-diff.py 6ae4a364b1e42ace9e25df047a2de70bdf4c4948

Prints, per function, the raw-byte hash groups (proves the byte-level
"three distinct pi_screen sequences" claim) and the whitespace/comment-
normalized hash groups (proves they are logically identical). Reads each
file via `git show <ref>:<path>` so it works from any later commit/worktree
without needing the deleted files to exist on disk.
"""
import hashlib
import re
import subprocess
import sys

FILES = [
    "src/bin/ingest_adventurers_guide_spells.rs",
    "src/bin/ingest_inner_sea_gods_spells.rs",
    "src/bin/ingest_inner_sea_setting_spells.rs",
    "src/bin/ingest_occult_adventures_spells.rs",
    "src/bin/ingest_ultimate_combat_spells.rs",
    "src/bin/ingest_ultimate_magic_spells.rs",
    "src/bin/ingest_ultimate_wilderness_spells.rs",
]


def git_show(ref: str, path: str) -> str:
    return subprocess.run(
        ["git", "show", f"{ref}:{path}"], capture_output=True, text=True, check=True
    ).stdout


def extract_fn(text: str, fname: str):
    m = re.search(r"\nfn " + re.escape(fname) + r"\(", text)
    if not m:
        return None
    start = m.start() + 1
    brace_start = text.index("{", start)
    depth = 0
    i = brace_start
    while i < len(text):
        if text[i] == "{":
            depth += 1
        elif text[i] == "}":
            depth -= 1
            if depth == 0:
                return text[start : i + 1]
        i += 1
    return None


def normalize(body: str) -> str:
    lines = []
    for line in body.split("\n"):
        code = line.split("//")[0].strip()
        if code:
            lines.append(code)
    return re.sub(r"\s+", " ", " ".join(lines))


def main():
    if len(sys.argv) != 2:
        print(f"usage: {sys.argv[0]} <git-ref-with-the-seven-binaries>", file=sys.stderr)
        sys.exit(2)
    ref = sys.argv[1]

    for fnname in ("pi_screen", "min_level"):
        print(f"##### {fnname} #####")
        raw_groups: dict[str, list[str]] = {}
        norm_groups: dict[str, list[str]] = {}
        for f in FILES:
            try:
                text = git_show(ref, f)
            except subprocess.CalledProcessError:
                print(f"  {f}: not found at {ref} (skipped)")
                continue
            body = extract_fn(text, fnname)
            if body is None:
                print(f"  {f}: no `{fnname}` in this file")
                continue
            raw_h = hashlib.sha256(body.encode()).hexdigest()[:8]
            norm_h = hashlib.sha256(normalize(body).encode()).hexdigest()[:8]
            raw_groups.setdefault(raw_h, []).append(f)
            norm_groups.setdefault(norm_h, []).append(f)
        print("  raw-byte hash groups:")
        for h, fs in raw_groups.items():
            print(f"    {h}: {fs}")
        print("  whitespace/comment-normalized hash groups:")
        for h, fs in norm_groups.items():
            print(f"    {h}: {fs}")
        print()


if __name__ == "__main__":
    main()
