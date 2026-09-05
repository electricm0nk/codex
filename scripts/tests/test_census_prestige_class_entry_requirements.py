#!/usr/bin/env python3
"""Regression test for `scripts/census_prestige_class_entry_requirements.py`.

SD-34 wave 44 (`decisions.md` §22, WAVE 43 UPDATE's own 191-unit finding):
`extract()`'s original `prestige_names.setdefault(name, path)` keyed purely
by display name across the full oracle, so whichever source file `os.walk`
happened to visit FIRST for a given class name won -- even when that file
sat under a book this repo has never ingested. When a class name also
appears in an older, un-ingested predecessor book (e.g.
`psionics_unleashed`/`psionics_expanded` predecessors of the ingested
`ultimate_psionics`), a filesystem-order race could let the non-ingested
file win, silently dropping the real ingested entry forever (the script's
own `if matched_book is None: continue` then discards it with no warning).

This test reproduces the exact collision shape against a small synthetic
corpus, rather than the live 158-book oracle, so it stays fast and
deterministic. It also directly forces the walk order that triggers the old
bug (non-ingested file visited before the ingested one) by monkeypatching
`os.walk`, so this test would have failed against the pre-fix script
regardless of what a real filesystem's directory order happens to be.
"""

import json
import os
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import census_prestige_class_entry_requirements as CENSUS  # noqa: E402


def _touch(path: str, content: str = "") -> None:
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as fh:
        fh.write(content)


NON_INGESTED_LINE = (
    "CLASS:Thrallherd\tHD:6\tTYPE:PC.Prestige.Psionic\tMAXLEVEL:10\n"
    "CLASS:Thrallherd\tPRESKILL:1,Knowledge (Psionics)=1\n"
)

INGESTED_LINE = (
    "CLASS:Thrallherd\tHD:6\tTYPE:PC.Prestige.Psionic\tMAXLEVEL:10\n"
    "CLASS:Thrallherd\tPRESKILL:2,Knowledge (Psionics)=5,Diplomacy=4\n"
)


def _build_corpus(tmp: str) -> tuple[Path, Path, Path, Path]:
    """A synthetic oracle with the exact collision shape: the SAME display
    name ("Thrallherd") appears both in a non-ingested predecessor book
    (`psionics_unleashed`) and in the real ingested book
    (`ultimate_psionics`), each with different (and distinguishable) PRE
    tokens so the test can tell which one the script actually kept."""
    repo_root = Path(tmp) / "repo"
    corpus_root = Path(tmp) / "oracle"

    # Only `ultimate_psionics` is ingested -- matches the real repo shape
    # where `psionics_unleashed`/`psionics_expanded` are NOT under
    # data/corpus/ at all.
    _touch(str(repo_root / "data" / "corpus" / "ultimate_psionics" / ".keep"))

    non_ingested_path = (
        corpus_root
        / "dreamscarred_press"
        / "psionics_unleashed"
        / "psionics_unleashed_classes_prestige.lst"
    )
    ingested_path = (
        corpus_root / "dreamscarred_press" / "ultimate_psionics" / "up_classes.lst"
    )
    _touch(str(non_ingested_path), NON_INGESTED_LINE)
    _touch(str(ingested_path), INGESTED_LINE)

    return repo_root, corpus_root, non_ingested_path, ingested_path


def _walk_forcing_order(corpus_root: Path, first_dir_name: str):
    """A drop-in replacement for `os.walk` that visits `first_dir_name`
    (relative to corpus_root) before every other subtree, so the test can
    force the exact race condition regardless of the real filesystem's own
    directory iteration order."""
    real_walk = os.walk

    def _walk(top, *args, **kwargs):
        if Path(top) != corpus_root:
            yield from real_walk(top, *args, **kwargs)
            return
        entries = sorted(Path(top).iterdir())
        ordered = sorted(
            entries, key=lambda p: (p.name != first_dir_name, p.name)
        )
        dirnames = [p.name for p in ordered if p.is_dir()]
        filenames = [p.name for p in ordered if p.is_file()]
        yield (str(top), dirnames, filenames)
        for d in dirnames:
            yield from real_walk(str(Path(top) / d))

    return _walk


class IngestedBookAlwaysWinsTest(unittest.TestCase):
    def test_ingested_book_wins_when_non_ingested_visited_first(self):
        """Reproduces the exact wave-44 bug shape: `os.walk` visits the
        non-ingested predecessor book before the real ingested book. Before
        the fix, `setdefault` would keep the FIRST (non-ingested) path and
        the entry would be silently dropped. After the fix, the ingested
        book's own PRE tokens must win."""
        with tempfile.TemporaryDirectory() as tmp:
            repo_root, corpus_root, _non_ingested, _ingested = _build_corpus(tmp)
            with mock.patch.object(
                CENSUS.os,
                "walk",
                _walk_forcing_order(corpus_root, "psionics_unleashed"),
            ):
                result = CENSUS.extract(corpus_root, repo_root)

            self.assertIn("Thrallherd", result)
            entry = result["Thrallherd"]
            self.assertEqual(entry["source_book"], "ultimate_psionics")
            self.assertEqual(
                entry["pre_tokens"],
                ["PRESKILL:2,Knowledge (Psionics)=5,Diplomacy=4"],
            )

    def test_ingested_book_wins_when_ingested_visited_first(self):
        """Same collision, opposite walk order -- the result must be
        identical either way, proving the outcome no longer depends on
        filesystem iteration order at all."""
        with tempfile.TemporaryDirectory() as tmp:
            repo_root, corpus_root, _non_ingested, _ingested = _build_corpus(tmp)
            with mock.patch.object(
                CENSUS.os,
                "walk",
                _walk_forcing_order(corpus_root, "ultimate_psionics"),
            ):
                result = CENSUS.extract(corpus_root, repo_root)

            self.assertIn("Thrallherd", result)
            entry = result["Thrallherd"]
            self.assertEqual(entry["source_book"], "ultimate_psionics")
            self.assertEqual(
                entry["pre_tokens"],
                ["PRESKILL:2,Knowledge (Psionics)=5,Diplomacy=4"],
            )

    def test_non_ingested_only_class_is_still_dropped(self):
        """A class that ONLY exists in a non-ingested book (no collision)
        must still be excluded -- the fix must not accidentally start
        keeping non-ingested-only entries."""
        with tempfile.TemporaryDirectory() as tmp:
            repo_root = Path(tmp) / "repo"
            corpus_root = Path(tmp) / "oracle"
            _touch(str(repo_root / "data" / "corpus" / "ultimate_psionics" / ".keep"))
            _touch(
                str(
                    corpus_root
                    / "dreamscarred_press"
                    / "psionics_unleashed"
                    / "psionics_unleashed_classes_prestige.lst"
                ),
                NON_INGESTED_LINE,
            )
            result = CENSUS.extract(corpus_root, repo_root)
            self.assertNotIn("Thrallherd", result)

    def test_output_json_matches_the_committed_fixture_shape(self):
        """Sanity check that `main()`'s payload shape (a `_comment` plus a
        sorted `entries` list keyed by `display_name`) still round-trips,
        since the committed fixture is the actual load-bearing artifact."""
        with tempfile.TemporaryDirectory() as tmp:
            repo_root, corpus_root, _non_ingested, _ingested = _build_corpus(tmp)
            result = CENSUS.extract(corpus_root, repo_root)
            payload = {
                "_comment": "test",
                "entries": [result[name] for name in sorted(result)],
            }
            # Must be JSON-serializable and round-trip cleanly.
            reloaded = json.loads(json.dumps(payload))
            self.assertEqual(reloaded["entries"][0]["display_name"], "Thrallherd")


if __name__ == "__main__":
    unittest.main()
