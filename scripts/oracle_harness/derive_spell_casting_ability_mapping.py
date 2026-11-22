#!/usr/bin/env python3
"""Derive the PF1 class -> governing spellcasting ability mapping directly
from the pinned PCGen oracle checkout (`scripts/pcgen-oracle-pin.env`).

SD-33 remediation wave 2 (`spell-remainder` slice, `AT-33-E5-001`/`-002`'s
named blocker: "no casting-ability mapping"). Reads every real
`CLASS:<Name> ... SPELLSTAT:<ABBREV> ...` declaration in the pinned
checkout's own `data/pathfinder/paizo/roleplaying_game/*/*_classes.lst`
files (official Paizo class data only -- never homebrew/3rd-party, matching
this repo's own corpus ingestion scope) and writes the resulting mapping to
`spell_casting_ability_mapping.json` alongside this script.

Never hand-rolled or transcribed from memory -- every entry traces to a real
line in the pinned checkout, and the `sources` field on each entry names the
exact file the declaration came from so a reader can re-derive it directly:

    grep -n "^CLASS:<Name>" $PCGEN_REPO_DIR/<source file>

Run: `python3 scripts/oracle_harness/derive_spell_casting_ability_mapping.py`
(resolves the checkout via `$PCGEN_REPO_DIR`, falling back to
`$HOME/workspace/repos/pcgen` -- `scripts/fetch-pcgen-oracle.sh`'s own
default -- never a hardcoded literal path in this file, per `AGENTS.md`'s
"PCGen oracle... never cited by literal local path" rule).
"""
from __future__ import annotations

import glob
import json
import os
import re
import subprocess
import sys

ABBREV_TO_ABILITY = {
    "STR": "Strength",
    "DEX": "Dexterity",
    "CON": "Constitution",
    "INT": "Intelligence",
    "WIS": "Wisdom",
    "CHA": "Charisma",
}


def pcgen_repo_dir() -> str:
    return os.environ.get("PCGEN_REPO_DIR") or os.path.expanduser("~/workspace/repos/pcgen")


def pinned_sha(repo_dir: str) -> str | None:
    try:
        out = subprocess.run(
            ["git", "-C", repo_dir, "rev-parse", "HEAD"],
            capture_output=True, text=True, check=True,
        )
        return out.stdout.strip()
    except Exception:
        return None


def derive(repo_dir: str) -> dict:
    pattern = os.path.join(repo_dir, "data/pathfinder/paizo/roleplaying_game/*/*_classes*.lst")
    class_files = sorted(glob.glob(pattern))
    # Official Paizo base-class files only: exclude PFS-legal variant lists
    # and monster/companion class stat blocks, neither of which is a player
    # spellcasting class this mapping needs to cover.
    class_files = [f for f in class_files if "_pfs" not in f and "companion" not in f]

    mapping: dict[str, str] = {}
    sources: dict[str, list] = {}
    spellstat_re = re.compile(r"SPELLSTAT:([A-Z]+)")

    for path in class_files:
        rel = os.path.relpath(path, repo_dir)
        with open(path, encoding="utf-8", errors="replace") as fh:
            for line in fh:
                if not line.startswith("CLASS:") or "SPELLSTAT:" not in line:
                    continue
                fields = line.rstrip("\n").split("\t")
                class_name = fields[0][len("CLASS:"):].strip()
                m = spellstat_re.search(line)
                if not m:
                    continue
                abbrev = m.group(1)
                ability = ABBREV_TO_ABILITY.get(abbrev)
                sources.setdefault(class_name, []).append({"file": rel, "spellstat": abbrev})
                # First declaration wins (a class's SPELLSTAT is declared once
                # in its own book; a later `.MOD` re-stating the same value is
                # not a disagreement -- see `mapping_disagreements` below,
                # which checks for a REAL differing value, not a duplicate).
                mapping.setdefault(class_name, ability)

    return {"mapping": mapping, "sources": sources}


def mapping_disagreements(sources: dict[str, list]) -> list[dict]:
    """Any class whose own sources disagree on the ability abbreviation --
    a genuine multi-declaration conflict, not just a repeat."""
    out = []
    for cls, entries in sources.items():
        abbrevs = {e["spellstat"] for e in entries}
        if len(abbrevs) > 1:
            out.append({"class": cls, "abbrevs": sorted(abbrevs), "sources": entries})
    return out


def main() -> int:
    repo_dir = pcgen_repo_dir()
    if not os.path.isdir(repo_dir):
        print(f"derive_spell_casting_ability_mapping: no checkout at {repo_dir!r}. "
              f"Bootstrap with scripts/fetch-pcgen-oracle.sh first.", file=sys.stderr)
        return 2

    result = derive(repo_dir)
    disagreements = mapping_disagreements(result["sources"])

    out = {
        "derived_from": (
            "PCGEN_REPO_DIR (resolved at run time, never hardcoded) -- "
            "data/pathfinder/paizo/roleplaying_game/*/*_classes.lst CLASS: lines "
            "carrying a SPELLSTAT: token. Re-derive: "
            "python3 scripts/oracle_harness/derive_spell_casting_ability_mapping.py"
        ),
        "pcgen_oracle_sha_at_derivation": pinned_sha(repo_dir),
        "class_count": len(result["mapping"]),
        "mapping": dict(sorted(result["mapping"].items())),
        "sources": dict(sorted(result["sources"].items())),
        "cross_check_against_engine_casting_ability_for_class": {
            "note": (
                "src/rules_core/spellbook.rs::casting_ability_for_class (private, "
                "7 classes: wizard/cleric/druid/ranger/sorcerer/bard/paladin -> "
                "Intelligence/Wisdom/Wisdom/Wisdom/Charisma/Charisma/Charisma) "
                "cross-checked against this mapping's Wizard/Cleric/Druid/Ranger/"
                "Sorcerer/Bard/Paladin entries."
            ),
            "engine_seven_class_map": {
                "Wizard": "Intelligence",
                "Cleric": "Wisdom",
                "Druid": "Wisdom",
                "Ranger": "Wisdom",
                "Sorcerer": "Charisma",
                "Bard": "Charisma",
                "Paladin": "Charisma",
            },
            "agrees": all(
                result["mapping"].get(c) == a
                for c, a in {
                    "Wizard": "Intelligence", "Cleric": "Wisdom", "Druid": "Wisdom",
                    "Ranger": "Wisdom", "Sorcerer": "Charisma", "Bard": "Charisma",
                    "Paladin": "Charisma",
                }.items()
            ),
        },
        "multi_declaration_disagreements": disagreements,
    }

    out_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "spell_casting_ability_mapping.json")
    with open(out_path, "w") as f:
        json.dump(out, f, indent=2, sort_keys=False)
        f.write("\n")

    print(f"derive_spell_casting_ability_mapping: {len(result['mapping'])} classes -> {out_path}")
    print(f"cross-check against engine's 7-class map: agrees={out['cross_check_against_engine_casting_ability_for_class']['agrees']}")
    if disagreements:
        print(f"WARNING: {len(disagreements)} class(es) have conflicting SPELLSTAT declarations across sources: {disagreements}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
