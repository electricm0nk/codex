#!/usr/bin/env python3
"""SD-33 AT-33-E5-001 remediation -- batch `.pcg` generator for the
`spell_effect_probe_observed_computed_delta` slice of the `fixture-verified`
spell population (`fixture_verified_oracle_probe`'s `spell` array).

Attempt 1 hand-authored one `.pcg` per unit. This generator batches EVERY
spell that shares a casting class into ONE `.pcg` character, so one PCGen
JVM start (the dominant real cost, ~22s measured this cycle) verifies many
units at once -- the lever named in this remediation's dispatch brief
("A character can carry many units at once").

Input: the JSON `fixture_verified_oracle_probe --output <path>` writes.
Output, per casting class with >=1 unit:
  fixtures/fixture-spell-pcg/<class>.pcg
  fixtures/fixture-spell-oracle-txt/<class>.export.txt   (written by run_batch.sh)
Shared template: fixtures/fixture-spell-batch.txt.ftl (written once, not
per-class -- the SPELLMEM loop is class-index-0-generic, proven against the
real pinned oracle this cycle: `class=0`, `spellbook=0` ("Known Spells") is
where a `.pcg`'s bare `SPELLNAME:` lines land per the real
`code/src/java/pcgen/io/PCGVer2Creator.java` writer / matching examples in
`characters/SpecialWizard.pcg`, `code/testsuite/PCGfiles/pf_Paladin.pcg`.

Also writes `fixture-spell-batch.manifest.json`: per unit_id, which class
file it lives in and the (level, name) key to look it up by after the
export -- the join key `run_compare_spell_batch.py` uses.
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

BOOK_TO_CAMPAIGN = {
    "core_rulebook": "Core Rulebook",
    "advanced_players_guide": "Advanced Player's Guide",
    "advanced_class_guide": "Advanced Class Guide",
    "occult_adventures": "Occult Adventures",
    "ultimate_combat": "Ultimate Combat",
    "ultimate_intrigue": "Ultimate Intrigue",
    "ultimate_magic": "Ultimate Magic",
    "ultimate_wilderness": "Ultimate Wilderness",
}

# Every class `.pcg` loads this FULL closure, not just the books its own
# spell subset happens to come from. Empirically discovered this cycle: with
# only `Core Rulebook` + `Advanced Class Guide` loaded, PCGen's own
# `_advanced_class_guide.pcc` PRECAMPAIGN chain (`INCLUDES=Ultimate Magic`)
# leaves an unresolved `%LIST` reference (`Prodigy (%LIST)`,
# `acg_abilities_class.lst:3865`) and the whole load throws
# `IllegalStateException` -- a pre-existing PCGen data-set cross-reference,
# nothing to do with any spell content. Reading every book's own PRECAMPAIGN
# line (`_occult_adventures.pcc`, `_ultimate_intrigue.pcc`,
# `_ultimate_wilderness.pcc`, `_ultimate_combat.pcc`, `_ultimate_magic.pcc`,
# `advanced_players_guide.pcc`) and taking the union is cheaper and safer
# than discovering each chain one `IllegalStateException` at a time.
ALWAYS_LOAD_CAMPAIGNS = [
    "Core Rulebook",
    "Advanced Player's Guide",
    "Advanced Class Guide",
    "Advanced Race Guide",
    "Ultimate Combat",
    "Ultimate Equipment",
    "Ultimate Magic",
    "Ultimate Intrigue",
    "Ultimate Wilderness",
    "Occult Adventures",
    "Bestiary",
    "Bestiary 2",
    "Bestiary 3",
]

TEMPLATE = """<#ftl encoding="UTF-8" strip_whitespace=true >
<#-- SD-33 AT-33-E5-001 remediation -- generic SPELLMEM batch export.
     spellbook=0 ("Known Spells", the default bare-SPELLNAME book) is
     constant, but the PCGen `class` index a character's ONE class lands on
     is NOT always 0 -- empirically found this cycle: Wizard/Cleric/Druid/
     Bard/Ranger's own single class landed on index 0, but Paladin's
     landed on index 1 (both `.pcg`s carry exactly one `CLASS:` line; PCGen
     evidently orders `SPELLLISTCLASS` by something other than declaration
     order this cycle did not chase down further). Looping `class` 0..2
     covers every case observed and costs nothing extra for the classes
     that land on 0 -- the higher indices simply report `count=0`. -->
<#assign spellbook = 0 />
<#list 0..2 as class>
<#list 0..9 as level>
<#assign n = pcvar('COUNT[SPELLSINBOOK.${class}.${spellbook}.${level}]') />
<#if (n > 0)>
<#list 0..(n-1) as spell>
SPELL.${class}.${level}.${spell}.NAME=${pcstring('SPELLMEM.${class}.${spellbook}.${level}.${spell}.NAME')}
SPELL.${class}.${level}.${spell}.DC=${pcstring('SPELLMEM.${class}.${spellbook}.${level}.${spell}.DC')}
</#list>
</#if>
</#list>
</#list>
"""

PCG_HEADER = """PCGVERSION:2.0

# System Information
{campaigns}
VERSION:6.09.08.RC1
GAMEMODE:Pathfinder_RPG
CHARACTERTYPE:PC
PURCHASEPOINTS:N
AUTOSPELLS:N

# Character Bio
CHARACTERNAME:E5 Spell Batch {class_human}
PLAYERNAME:sd-33-e5-fixture

# Character Attributes
STAT:STR|SCORE:10
STAT:DEX|SCORE:10
STAT:CON|SCORE:10
STAT:INT|SCORE:18
STAT:WIS|SCORE:10
STAT:CHA|SCORE:18
ALIGN:{align}
RACE:Human

# Character Class(es)
CLASS:{class_human}|LEVEL:20|SKILLPOOL:0

# Character Experience
EXPERIENCE:0
EXPERIENCETABLE:Medium

# Character Spells Information
"""

# Paladin is alignment-restricted (LG) by the real PF1 rule; every other
# probed class (Wizard/Cleric/Druid/Bard/Sorcerer/Ranger) is alignment-free
# for spellcasting purposes, so LN (Druid's own requirement: at least one
# Neutral component) is a safe default for the rest.
CLASS_ALIGNMENT = {"Paladin": "LG"}

SPELL_LINE = (
    "SPELLNAME:{name}|TIMES:1|CLASS:{class_human}|BOOK:Known Spells|"
    "SPELLLEVEL:{level}|SOURCE:[TYPE:CLASS|NAME:{class_human}]\n"
)


def main() -> int:
    if len(sys.argv) != 3:
        print("usage: fixture-generate-spell-batch.py <probe-output.json> <out-dir>", file=sys.stderr)
        return 2
    probe_path = Path(sys.argv[1])
    out_dir = Path(sys.argv[2])
    pcg_dir = out_dir / "fixture-spell-pcg"
    pcg_dir.mkdir(parents=True, exist_ok=True)

    data = json.loads(probe_path.read_text())
    spells = data["spell"]

    by_class: dict[str, list[dict]] = {}
    for row in spells:
        by_class.setdefault(row["class_human"], []).append(row)

    manifest = {"by_class": {}, "unit_count": len(spells)}
    (out_dir / "fixture-spell-batch.txt.ftl").write_text(TEMPLATE)

    for class_human, rows in sorted(by_class.items()):
        books = sorted({r["book"] for r in rows})
        missing = [b for b in books if b not in BOOK_TO_CAMPAIGN]
        if missing:
            raise SystemExit(f"no CAMPAIGN mapping for books {missing} (class {class_human})")
        campaigns = "\n".join(f"CAMPAIGN:{c}" for c in ALWAYS_LOAD_CAMPAIGNS)

        # Dedup by (level, name): PCGen distinguishes spells by (level, name)
        # within one spellbook/class -- two units sharing exactly one of
        # each would be genuinely ambiguous on the oracle side. Detect and
        # report rather than silently overwrite.
        seen: dict[tuple, str] = {}
        lines = []
        collisions = []
        for r in rows:
            key = (r["level"], r["name"])
            if key in seen and seen[key] != r["unit_id"]:
                collisions.append({"key": key, "units": [seen[key], r["unit_id"]]})
                continue
            seen[key] = r["unit_id"]
            lines.append(SPELL_LINE.format(name=r["name"], class_human=class_human, level=r["level"]))

        align = CLASS_ALIGNMENT.get(class_human, "LN")
        pcg_text = PCG_HEADER.format(campaigns=campaigns, class_human=class_human, align=align) + "".join(lines)
        pcg_text += "\n# Character Equipment\nMONEY:0\n"
        slug = class_human.lower()
        (pcg_dir / f"{slug}.pcg").write_text(pcg_text)

        manifest["by_class"][class_human] = {
            "pcg": f"fixture-spell-pcg/{slug}.pcg",
            "unit_ids": [r["unit_id"] for r in rows],
            "count": len(rows),
            "collisions": collisions,
        }

    (out_dir / "fixture-spell-batch.manifest.json").write_text(json.dumps(manifest, indent=2))
    total_collisions = sum(len(v["collisions"]) for v in manifest["by_class"].values())
    print(
        f"fixture-generate-spell-batch: {len(spells)} spell units -> "
        f"{len(by_class)} class files, {total_collisions} (level,name) collisions",
        file=sys.stderr,
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
