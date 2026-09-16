#!/usr/bin/env python3
"""Say the description argument in the rule's own words, not in PCGen's --
SD-35 Epic 6, `AT-35-E6-003-SWEEP` cycle 11.

Why this exists
---------------
`decisions.md` §1 (the sheet rule): a sheet line is a final number, dice in
final form, or the rule's words. `decisions.md` §11: nothing on the live side
reads a PCGen token.

Three shipped content tables still broke both at once. They are not internal
plumbing -- they are the words a player reads:

* `MonsterAbilityRecord::description` is printed verbatim.
* `MonsterAbilityRecord::description_variables` supplies the words that
  `%1`/`%2`/`%3` in that description are replaced with. It is indexed
  positionally by
  `src/rules_core/derived_evaluator_fixture_check.rs::monster_ability_save_dc`
  and served to the desktop crate by
  `apps/desktop/src-tauri/src/companion_catalog.rs`.

So a `"%CHOICE"` sitting in that array does not describe a choice -- it IS the
substitution token, printed where the chosen option's name belongs.

The three mechanisms, and the word each becomes
-----------------------------------------------
1. `"%CHOICE"` -> `"the chosen option"`   (5 entries, `bestiary`)
2. `"%LIST"`   -> `"the chosen option"`   (15 entries, `bestiary`)

   `%CHOICE` and `%LIST` are PCGen's two spellings of "the option this
   character chose when they took this ability". **This project already ships
   exactly one phrase for that**, written by the converter itself:
   `src/pcgen_import/sheet_rule/convert.rs::tag_word` returns the literal
   `"the chosen option"` for any qualifier containing `%LIST` or `%CHOICE`, and
   `bonus_target` uses the same reading for `SKILLRANK`. This script does not
   invent a vocabulary; it makes the tables say what the converter already
   says. `sheet_rule.rs::ProsePiece::ChoiceName` is the typed form of the same
   fact, and its doc comment states the same fallback ("unmade -> the choice
   lane's unmade words").

3. `"TYPE=Base"` -> `"Base"`   (2 entries, `inner_sea_world_guide`)

   Treerazer's two save-DC rows carry `["10+(HD/2)+CON", "TYPE=Base"]`. Slot 1
   is the DC formula the description's `%1` names; slot 2 is the DC's bonus
   TYPE, which neither description references (both read "DC %1" and stop).
   `tag_word` again already states the reading: it strips the `TYPE=` prefix
   and keeps the bare game word. The array keeps its length, so every
   positional index into it is unchanged.

4. `" DESC:&nl; "` -> `"&nl;"`   (4 occurrences in ONE description,
   `bestiary_3` "Flail Snail ~ Warp Magic")

   Upstream `b3_abilities_race.lst:869` states this ability as five `DESC:`
   tokens on one line; the ingest concatenated them and the repeated token
   NAME came along, so the player's sheet reads "...consult the following
   table. DESC:&nl; 1-3 Spell misfires...". Only the leaked `DESC:` marker is
   dropped. The `&nl;` entity beside it is left exactly as it stands, because
   that is what every other shipped description in this repo carries for a
   line break (`monster_codex/spell_list.rs`, `bestiary_6/monster_data.rs`,
   `beastiary1/companion_data.rs`, the three ACG feat rows
   `pcgen_desc.rs:225` names) and `pcgen_desc::PCGEN_ENTITIES` decodes it to a
   real newline. Repairing `&nl;` corpus-wide is a different mechanism with a
   different population; this cycle's receipt reports it as a discovery rather
   than half-doing it here.

What this script deliberately does NOT do
-----------------------------------------
* It does not touch `data/corpus/**`. The corpus record is the ingest
  literal and `corpus_literal_sweep` holds it byte-equal to the `.lst`; the
  record's own `source_file`/`source_line` fields on every row edited here
  remain the provenance path back to it.
* It does not change any array's LENGTH, so no positional read moves.
* It does not reword a description. Mechanism 4 deletes a token name and
  nothing else.

Gate: `tests/sd35_rendered_prose_carries_no_ingest_vocabulary.rs`, whose
`SCANNED` list this cycle extended to these three files. It re-derives the
count from the files rather than from a recorded figure.

Run:
    python3 .../AT-35-E6-003-SWEEP_cycle11_description_argument_words.py --check
    python3 .../AT-35-E6-003-SWEEP_cycle11_description_argument_words.py --apply
"""

import argparse
import os
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", ".."))

# The one phrase this project already ships for "the option this character
# chose" -- src/pcgen_import/sheet_rule/convert.rs::tag_word.
CHOSEN = "the chosen option"

# (relative path, exact old substring, exact new substring, expected occurrences)
EDITS = [
    # 1 + 2 -- the substitution tokens, as whole array entries so a token
    # named inside a description sentence can never be hit by accident.
    ("src/rules_core/rules_tables/bestiary/monster_data.rs", '"%CHOICE"', f'"{CHOSEN}"', 5),
    ("src/rules_core/rules_tables/bestiary/monster_data.rs", '"%LIST"', f'"{CHOSEN}"', 15),
    # 3 -- the bonus TYPE in slot 2, as the bare game word.
    ("src/rules_core/rules_tables/inner_sea_world_guide/monster_data.rs", '"TYPE=Base"', '"Base"', 2),
    # 4 -- the leaked repetition marker, and only it.
    ("src/rules_core/rules_tables/bestiary_3/monster_data.rs", " DESC:&nl; ", "&nl; ", 4),
]


def apply_edits(write: bool) -> int:
    by_file: dict[str, list[tuple[str, str, int]]] = {}
    for rel, old, new, count in EDITS:
        by_file.setdefault(rel, []).append((old, new, count))

    failures: list[str] = []
    applied = 0
    for rel, edits in by_file.items():
        path = os.path.join(REPO, rel)
        with open(path, encoding="utf-8") as fh:
            text = fh.read()
        original = text
        for old, new, expected in edits:
            found = text.count(old)
            if found != expected:
                failures.append(f"{rel}: expected {expected} of {old!r}, found {found}")
                continue
            text = text.replace(old, new)
            applied += found
            print(f"  {rel}: {found} x {old!r} -> {new!r}")
        if write and text != original:
            with open(path, "w", encoding="utf-8") as fh:
                fh.write(text)

    if failures:
        for line in failures:
            print(f"MISMATCH {line}", file=sys.stderr)
        return 1
    print(f"{'applied' if write else 'would apply'}={applied}")
    return 0


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--check", action="store_true", help="report what would change; write nothing")
    group.add_argument("--apply", action="store_true", help="rewrite the three tables in place")
    args = parser.parse_args(argv)
    return apply_edits(write=args.apply)


if __name__ == "__main__":
    sys.exit(main())
