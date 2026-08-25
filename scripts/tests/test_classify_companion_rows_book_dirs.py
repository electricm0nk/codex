"""Self-test for `scripts/classify_companion_rows.py::book_dirs`.

`docs/work-inventory.json`'s own `corpus_root` field records the ABSOLUTE PATH
of whatever worktree last regenerated it, which is exactly the "PCGen oracle
cited by literal local path" shape `AGENTS.md` forbids. `book_dirs()` used to
read that field directly (`inv["corpus_root"]`) instead of calling its own
`corpus_root()` helper (already `PCGEN_CORPUS_ROOT`-env-var-aware, and already
used correctly by the sibling `classify_monster_ability_rows.py::book_dirs`).
The result: `transcribe_companion_tables.py` raised `FileNotFoundError` in
every fresh worktree whose `PCGEN_CORPUS_ROOT` pointed somewhere the committed
JSON's stale absolute path did not — which is every fresh worktree, since the
oracle slot is git-ignored and re-fetched per-worktree.

**Read-only, no real oracle dependency.** Builds a scratch corpus directory
tree and a scratch `work-inventory.json` whose `corpus_root` field is a
deliberately WRONG, nonexistent path, then asserts `book_dirs()` still finds
the books because it must resolve against `PCGEN_CORPUS_ROOT`, not the JSON
field.

Run: python3 -m unittest scripts.tests.test_classify_companion_rows_book_dirs
"""
from __future__ import annotations

import importlib.util
import json
import os
import pathlib
import shutil
import tempfile
import unittest

_MODULE_PATH = (
    pathlib.Path(__file__).resolve().parent.parent / "classify_companion_rows.py"
)


def _load_module():
    spec = importlib.util.spec_from_file_location("classify_companion_rows", _MODULE_PATH)
    mod = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    spec.loader.exec_module(mod)
    return mod


class BookDirsRespectsPcgenCorpusRootEnvVar(unittest.TestCase):
    def setUp(self) -> None:
        self.scratch = tempfile.mkdtemp(prefix="companion_book_dirs_test_")
        # Mirrors the real shape: PCGEN_CORPUS_ROOT is the `pcgen/data`
        # directory; the roleplaying_game tier and each `additional_book_dirs`
        # entry sit under it.
        self.real_corpus_root = os.path.join(self.scratch, "real_corpus", "data")
        rpg_dir = os.path.join(
            self.real_corpus_root, "pathfinder", "paizo", "roleplaying_game"
        )
        os.makedirs(os.path.join(rpg_dir, "bestiary_4"))
        os.makedirs(os.path.join(rpg_dir, "bestiary_5"))
        extra_dir = os.path.join(
            self.real_corpus_root, "pathfinder", "dreamscarred_press", "ultimate_psionics"
        )
        os.makedirs(extra_dir)

        # A scratch `docs/work-inventory.json` whose `corpus_root`/
        # `additional_book_dirs` fields carry a DIFFERENT worktree's stale
        # absolute prefix -- the shape a committed value takes once the
        # worktree that wrote it is gone, ending in the same `/data/...`
        # relative structure the real corpus uses.
        self._orig_cwd = os.getcwd()
        os.chdir(self.scratch)
        os.makedirs("docs", exist_ok=True)
        stale_prefix = os.path.join(self.scratch, "worktree-that-no-longer-exists", "data")
        with open("docs/work-inventory.json", "w", encoding="utf-8") as fh:
            json.dump(
                {
                    "corpus_root": os.path.join(
                        stale_prefix, "pathfinder", "paizo", "roleplaying_game"
                    ),
                    "additional_book_dirs": [
                        os.path.join(
                            stale_prefix,
                            "pathfinder",
                            "dreamscarred_press",
                            "ultimate_psionics",
                        )
                    ],
                    "units": [],
                },
                fh,
            )

        self._orig_env = os.environ.get("PCGEN_CORPUS_ROOT")
        os.environ["PCGEN_CORPUS_ROOT"] = self.real_corpus_root

        self.mod = _load_module()

    def tearDown(self) -> None:
        os.chdir(self._orig_cwd)
        shutil.rmtree(self.scratch, ignore_errors=True)
        if self._orig_env is None:
            os.environ.pop("PCGEN_CORPUS_ROOT", None)
        else:
            os.environ["PCGEN_CORPUS_ROOT"] = self._orig_env

    def test_red_the_stale_json_field_alone_does_not_resolve(self) -> None:
        """RED proof: the JSON's own (wrong) corpus_root does not exist."""
        with open("docs/work-inventory.json", encoding="utf-8") as fh:
            inv = json.load(fh)
        self.assertFalse(os.path.isdir(inv["corpus_root"]))

    def test_book_dirs_finds_books_via_pcgen_corpus_root_env_var(self) -> None:
        dirs = self.mod.book_dirs()
        rpg_dir = os.path.join(
            self.real_corpus_root, "pathfinder", "paizo", "roleplaying_game"
        )
        self.assertIn("bestiary_4", dirs)
        self.assertIn("bestiary_5", dirs)
        self.assertEqual(dirs["bestiary_4"], os.path.join(rpg_dir, "bestiary_4"))
        self.assertEqual(dirs["bestiary_5"], os.path.join(rpg_dir, "bestiary_5"))

    def test_book_dirs_rebases_additional_book_dirs_too(self) -> None:
        """`additional_book_dirs` entries carry the same stale-worktree
        absolute prefix as `corpus_root` -- both must rebase, or a book living
        outside `roleplaying_game/` (e.g. `ultimate_psionics`, under
        `dreamscarred_press/`) resolves to a directory that does not exist."""
        dirs = self.mod.book_dirs()
        self.assertIn("ultimate_psionics", dirs)
        self.assertTrue(os.path.isdir(dirs["ultimate_psionics"]))


if __name__ == "__main__":
    unittest.main()
