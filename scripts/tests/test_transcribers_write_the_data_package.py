"""SD-37 E4a.4a: the two Python transcribers write the `rules_tables` data package.

E4a.4 removed the compiled `src/rules_core/rules_tables/`. These tests pin the conversions that used
to be applied BY HAND to the generated Rust tables after the transcribers ran (SD-35
`AT-35-E6-003-SWEEP` cycles 9, 11 and 13) and now run inside the transcribers, so a re-run
reproduces the package instead of reverting them; and the companion transcriber's package write.

The byte-identity of every book against the shipped package is proven by re-running both
transcribers against the pinned oracle (the E4a.4a receipt), not here.

Run: python3 -m unittest scripts/tests/test_transcribers_write_the_data_package.py
"""
from __future__ import annotations

import importlib.util
import json
import pathlib
import tempfile
import unittest

_SCRIPTS = pathlib.Path(__file__).resolve().parent.parent


def _load(name: str):
    spec = importlib.util.spec_from_file_location(name, _SCRIPTS / f"{name}.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


ttc = _load("transcribe_companion_tables")
tmt = _load("transcribe_monster_tables")


class CompanionGuardsAreTyped(unittest.TestCase):
    """Cycle 9 / 13: a `PRE<FAMILY>:` guard ships as an `EffectCondition`."""

    def test_a_guard_becomes_an_effect_condition_and_rebuilds_verbatim(self) -> None:
        token = "PREABILITY:1,CATEGORY=Special Ability,Animal Trick ~ Attack II"
        guard = ttc.typed_guard(token)
        self.assertEqual(
            guard,
            {
                "negated": False,
                "family": "ABILITY",
                "items": [
                    {"facet": None, "value": "1"},
                    {"facet": "CATEGORY", "value": "Special Ability"},
                    {"facet": None, "value": "Animal Trick ~ Attack II"},
                ],
                "alternatives": [],
            },
        )
        self.assertEqual(ttc.rebuild_guard(guard), token)

    def test_a_bracketed_alternative_nests(self) -> None:
        token = "!PREVARLT:MasterLevel,7,[PRETEMPLATE:1,X]"
        guard = ttc.typed_guard(token)
        self.assertTrue(guard["negated"])
        self.assertEqual(guard["alternatives"][0]["family"], "TEMPLATE")
        self.assertEqual(ttc.rebuild_guard(guard), token)

    def test_a_non_guard_is_refused(self) -> None:
        with self.assertRaises(SystemExit):
            ttc.typed_guard("Scent")

    def test_a_guard_tail_leaves_the_formula(self) -> None:
        bonus = ttc.natural_attack_damage_bonus("Bite", "max(0,(STR/2))|PREVARLT:MasterLevel,7")
        self.assertEqual(bonus["formula"], "max(0,(STR/2))")
        self.assertEqual(bonus["conditions"][0]["family"], "VARLT")
        self.assertEqual(ttc.natural_attack_damage_bonus("Claw", "STR")["conditions"], [])

    def test_a_grant_splits_into_kind_mode_name_and_conditions(self) -> None:
        grant = ttc.ability_grant("FEAT|AUTOMATIC|CMB Output|PRELEVEL:MIN=2")
        self.assertEqual((grant["kind"], grant["mode"], grant["name"]), ("FEAT", "AUTOMATIC", "CMB Output"))
        self.assertEqual(grant["conditions"][0]["items"], [{"facet": "MIN", "value": "2"}])
        with self.assertRaises(SystemExit):
            ttc.ability_grant("FEAT|CMB Output")

    def test_an_external_ref_guard_gates_the_ability_before_it(self) -> None:
        refs, conditions = ttc.split_external_ref_guards(
            [
                "Animal Traits Output",
                "Scent",
                "Gylptodon Companion Natural Attack",
                "!PRETEMPLATE:1,Gylptodon Companion Advancement",
            ]
        )
        self.assertEqual(refs, ["Animal Traits Output", "Scent", "Gylptodon Companion Natural Attack"])
        self.assertEqual([c["ability"] for c in conditions], ["Gylptodon Companion Natural Attack"])
        self.assertEqual(
            ttc.rebuild_guard(conditions[0]["conditions"][0]),
            "!PRETEMPLATE:1,Gylptodon Companion Advancement",
        )
        self.assertEqual(ttc.split_external_ref_guards(["Scent"]), (["Scent"], []))


class MonsterDescriptionArgumentsSayTheRulesWords(unittest.TestCase):
    """Cycle 11: `%CHOICE`/`%LIST` -> the converter's phrase; `TYPE=Base` -> `Base`;
    a leaked ` DESC:` token name is dropped. No slot is added or removed."""

    def test_the_three_mechanisms(self) -> None:
        description, words = tmt.in_the_rules_words(
            "Consult the table. DESC:&nl; 1-3 Spell misfires.",
            ["%CHOICE", "%LIST", "TYPE=Base", "10+(HD/2)+CON"],
        )
        self.assertEqual(description, "Consult the table.&nl; 1-3 Spell misfires.")
        self.assertEqual(words, ["the chosen option", "the chosen option", "Base", "10+(HD/2)+CON"])

    def test_other_text_is_untouched(self) -> None:
        self.assertEqual(tmt.in_the_rules_words(None, ["CON", "%1"]), (None, ["CON", "%1"]))


class CompanionWriteBookTargetsThePackage(unittest.TestCase):
    def setUp(self) -> None:
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        self.root = self._tmp.name
        old_normalise, old_transcribe = ttc.normalise_package, ttc.transcribe
        ttc.normalise_package = lambda: None
        self.addCleanup(setattr, ttc, "normalise_package", old_normalise)
        self.addCleanup(setattr, ttc, "transcribe", old_transcribe)

    def test_tables_land_under_the_engine_module_dir_with_stem_citations(self) -> None:
        row = {"key": "Companion (Wolf)", "source_file": "cr_races_companion.lst", "source_line": 7}
        ttc.transcribe = lambda book: ttc.Transcription(
            tables={"COMPANIONS": [row], "COMPANION_ABILITIES": []}, notes=["a note"]
        )
        paths = ttc.write_book("core_rulebook", self.root)
        base = pathlib.Path(self.root, "data/rules_tables/crb/companion_data")
        self.assertEqual(paths, [str(base / "COMPANIONS.json"), str(base / "COMPANION_ABILITIES.json")])
        written = json.loads((base / "COMPANIONS.json").read_text())
        self.assertEqual(written["table"], "crb/companion_data/COMPANIONS")
        self.assertEqual(written["rows"][0]["source_file"], "cr_races_companion")
        self.assertFalse(pathlib.Path(self.root, "src").exists())

    def test_a_refused_transcription_writes_nothing(self) -> None:
        def _boom(book: str):
            raise SystemExit("refused")

        ttc.transcribe = _boom
        with self.assertRaises(SystemExit):
            ttc.write_book("core_rulebook", self.root)
        self.assertFalse(pathlib.Path(self.root, "data").exists())


if __name__ == "__main__":
    unittest.main()
