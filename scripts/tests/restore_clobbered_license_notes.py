#!/usr/bin/env python3
"""Restore the `screening_method_note` the companion generator clobbered.

`src/bin/gen_book_cache.rs`'s `gen_companion_book` preserved a prior
`license_declaration` but replaced the prior `screening_method_note` outright,
so a book that had already been ingested by another lane lost every earlier
pass's account of how its records were screened. `decisions.md §54.4`.

The generator is fixed to compose. This script puts the lost text back into the
three books it was taken from, reading it out of the commit that immediately
preceded the companion write rather than retyping it, so the restored note is
the original bytes and not a paraphrase. Running the generator afterwards
appends this lane's pass to the restored text.

Usage::

    python3 scripts/tests/restore_clobbered_license_notes.py

Idempotent: a book whose note already starts with the restored text is left
alone.
"""

from __future__ import annotations

import json
import subprocess
import sys

# book -> the commit whose version of the file still carries the pre-companion
# note. Each is the parent of the companion commit that overwrote it, found
# with `git log --oneline -- data/corpus/<book>/LICENSE.json`.
SOURCES = {
    "monster_codex": "bac2f569~1",
    "horror_adventures": "bac2f569~1",
    "beastiary": "HEAD",
}


def prior_note(book: str, commit: str) -> str:
    path = f"data/corpus/{book}/LICENSE.json"
    raw = subprocess.run(
        ["git", "show", f"{commit}:{path}"],
        capture_output=True,
        text=True,
        check=True,
    ).stdout
    return json.loads(raw)["screening_method_note"]


def main() -> None:
    changed = 0
    for book, commit in SOURCES.items():
        path = f"data/corpus/{book}/LICENSE.json"
        with open(path, encoding="utf-8") as handle:
            current = json.load(handle)
        restored = prior_note(book, commit)
        if current["screening_method_note"].startswith(restored):
            print(f"{book}: already carries its prior note")
            continue
        current["screening_method_note"] = restored
        with open(path, "w", encoding="utf-8") as handle:
            json.dump(current, handle, indent=2, sort_keys=True)
            handle.write("\n")
        print(f"{book}: prior note restored from {commit}")
        changed += 1
    print(f"{changed} restored; re-run gen_book_cache for these books to compose this lane's pass")
    return 0


if __name__ == "__main__":
    sys.exit(main())
