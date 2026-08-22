#!/usr/bin/env python3
"""Extract real PCGen `CLASS:` entry-requirement (`PRE*`) tokens for every
prestige class whose source book this repo has actually ingested into
`data/corpus/`.

SD-32 Epic 3 (`epic-3-class-reachability`, `acceptance-and-verification.md`
AT-32-E3-001): "The 77 prestige classes have entry-requirement gating that
exists nowhere in the codebase today". This script is the re-derive command
for the population this cycle actually wires gating for, and its output
(`tests/fixtures/rules_core/prestige-class-entry-requirements.json`) is the
fixture `src/rules_core/pilot_compute/prestige_class_entry_gate.rs` loads at
compile time via `include_str!`.

Method, mechanical rather than curated:

1. Walk every `.lst` file under `$PCGEN_CORPUS_ROOT` (the pinned oracle).
2. A `CLASS:<Name>` line whose tab-delimited fields include a `TYPE:` value
   containing `Prestige` names a prestige class.
3. For every `CLASS:<Name>` line in the SAME source file (PCGen splits one
   class's tokens across multiple `CLASS:<Name>` rows), collect every
   `PRE*`-family field (skipping `PREREQ*`, which is a different PCGen
   directive family entirely, not an entry-requirement token).
4. Keep only classes whose source file sits under a book directory this
   repo has actually ingested (`data/corpus/<book>/` exists) -- gating logic
   for a book with no ingested corpus data at all would be untestable
   fiction, not a real mechanism.

Run: `PCGEN_CORPUS_ROOT=<oracle>/data python3 scripts/census_prestige_class_entry_requirements.py`
(or let the script resolve `$PCGEN_CORPUS_ROOT`/`$PCGEN_REPO_DIR` itself, matching every other
corpus command in this bundle).

The **77** figure quoted in `epic-breakdown.md` Epic 3 and
`acceptance-and-verification.md` AT-32-E3-001 is the corpus-wide (all 158
oracle books) `TYPE:...Prestige` population; this script's own run found
**131** such names oracle-wide and **62** of them anchored in a book this
repo has ingested (`data/corpus/<book>/` exists) -- the honest, buildable
subset, and the one the fixture this script writes actually covers. See the
cycle receipt this script's own commit shipped with for the re-derive
command and the retro correction logged against the 77 figure.
"""

from __future__ import annotations

import json
import os
import re
import sys
from pathlib import Path


def resolve_corpus_root() -> Path:
    env = os.environ.get("PCGEN_CORPUS_ROOT")
    if env:
        return Path(env)
    repo_dir = os.environ.get("PCGEN_REPO_DIR")
    if repo_dir:
        return Path(repo_dir) / "data"
    raise SystemExit(
        "PCGEN_CORPUS_ROOT (or PCGEN_REPO_DIR) is not set -- export it per "
        "workflow-instruction.md §2.1 before running this script."
    )


def ingested_books(repo_root: Path) -> set[str]:
    corpus_dir = repo_root / "data" / "corpus"
    return {p.name for p in corpus_dir.iterdir() if p.is_dir()}


def slugify(name: str) -> str:
    """Matches `ApgClassId::name`'s convention: lowercase, spaces and
    non-alphanumerics collapsed to single underscores, no leading/trailing
    underscore."""
    slug = re.sub(r"[^a-z0-9]+", "_", name.lower())
    return slug.strip("_")


def extract(corpus_root: Path, repo_root: Path) -> dict[str, dict]:
    books = ingested_books(repo_root)
    prestige_names: dict[str, Path] = {}
    class_lines: dict[tuple[Path, str], list[str]] = {}

    for dirpath, _dirs, files in os.walk(corpus_root):
        for fn in files:
            if not fn.endswith(".lst"):
                continue
            path = Path(dirpath) / fn
            try:
                text = path.read_text(encoding="utf-8", errors="replace")
            except OSError:
                continue
            for line in text.splitlines():
                if not line.startswith("CLASS:"):
                    continue
                fields = line.split("\t")
                name = fields[0][len("CLASS:"):]
                class_lines.setdefault((path, name), []).append(line)
                if re.search(r"TYPE:[^\t]*Prestige", line):
                    prestige_names.setdefault(name, path)

    result: dict[str, dict] = {}
    for name, path in sorted(prestige_names.items()):
        try:
            rel = path.relative_to(corpus_root)
        except ValueError:
            rel = path
        parts = rel.parts
        matched_book = next((p for p in parts if p in books), None)
        if matched_book is None:
            continue
        pre_tokens = []
        for line in class_lines.get((path, name), []):
            for field in line.split("\t")[1:]:
                bare = field.lstrip("!")
                if bare.startswith("PRE") and not bare.startswith("PREREQ"):
                    pre_tokens.append(field)
        if not pre_tokens:
            continue
        result[name] = {
            "class_id": f"class:{slugify(name)}",
            "display_name": name,
            "source_book": matched_book,
            "source_file": str(rel),
            "pre_tokens": pre_tokens,
        }
    return result


def main() -> int:
    repo_root = Path(__file__).resolve().parent.parent
    corpus_root = resolve_corpus_root()
    result = extract(corpus_root, repo_root)

    out_path = repo_root / "tests" / "fixtures" / "rules_core" / "prestige-class-entry-requirements.json"
    out_path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "_comment": (
            "Generated by scripts/census_prestige_class_entry_requirements.py. "
            "Do not hand-edit -- re-run the script against the pinned oracle instead."
        ),
        "entries": [result[name] for name in sorted(result)],
    }
    out_path.write_text(json.dumps(payload, indent=2, sort_keys=False) + "\n", encoding="utf-8")

    print(f"population={len(result)}", file=sys.stderr)
    print(f"wrote {out_path}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
