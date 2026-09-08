#!/usr/bin/env python3
"""The sheet-rule oracle parity run (SD-35 AT-35-E2-005; `technical-design.md §1`,
"Verification, with PCGen as the oracle"; `blockers.md` B1).

Three subcommands, run in order:

    python3 scripts/oracle_harness/sheet_parity.py roster  --out <dir>
        Writes the fixture roster: for every member both the engine's character input
        (`<name>.txt`, the `key=value` format `tests/fixtures/rules_core/*_input.txt` uses) and
        PCGen's `.pcg` for the same character. The deterministic Human Fighter 1 fixture and
        its GE-05 `.pcg` twin are copied in as-is.

    python3 scripts/oracle_harness/sheet_parity.py export  --roster <dir> --out <dir> [--jobs 3]
        Runs PCGen's BatchExporter (the pinned checkout, `charbuild_remainder_run_one.sh`) over
        every `.pcg` with `sheet-totals.txt.ftl`, writing `<name>.txt` exports. Read-only for
        the repository; a long job for the isolated worker.

    python3 scripts/oracle_harness/sheet_parity.py compare --ours <ours.json> --exports <dir> \\
        --output <parity.json>
        Joins `cargo run --release --bin sheet_rule_parity`'s output to the exports and prints
        `compared=<n> agree=<n> disagree=<n> unverifiable=<n> PCGEN_ORACLE_SHA=<sha>`. Exit 0
        always -- the counts are the receipt's figures; a disagreement is named, not hidden.

What is compared (every rendered `Number` line the live evaluator produced for the roster,
`lines` family; the chassis totals those lines feed, `chassis` family):

  * a line whose rule feeds a sheet total PCGen exports a component for is folded with its
    siblings on the same target (PCGen's `BonusManager` fold: same-type takes the max unless
    the type stacks) and compared with that component -- `SKILL.<n>.MISC`, the four
    non-armor AC parts, `CHECK.<n>.MISC` less the ability modifier, `INITIATIVEMISC`,
    `ATTACK.MELEE.MISC`, `BAB`, `CHECK.<n>.BASE`, `MOVE.<n>.RATE`, `DR`, `STAT.<n>.SCORE`;
  * a line with no such component (a DC, a caster level, uses per day, a plain number) is
    compared with the numbers PCGen printed in the same-named ability's substituted
    `DESCRIPTION` (`SA.<n>.DESC` / `FEAT.<n>.DESC`): agree when our number is among them;
  * a line PCGen exports nothing for is `unverifiable` with the reason named.

This is a tool-side script: it reads PCGen's export. Nothing here is imported by live code.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(os.path.dirname(HERE))
RUN_ONE = os.path.join(HERE, "charbuild_remainder_run_one.sh")
FTL = os.path.join(HERE, "sheet-totals.txt.ftl")
PIN_FILE = os.path.join(os.path.dirname(HERE), "pcgen-oracle-pin.env")
DETERMINISTIC_FIXTURE = os.path.join(
    REPO, "tests", "fixtures", "rules_core", "pf1_human_fighter_level1_ge06_deterministic_input.txt"
)
DETERMINISTIC_PCG = os.path.join(
    REPO,
    "docs",
    "release",
    "GE-05-oracle-validation-and-parity-harness",
    "artifacts",
    "pf1-crb-human-fighter-level1-provisional-ge05-e1-f2.pcg",
)

# ---------------------------------------------------------------------------
# The roster
# ---------------------------------------------------------------------------

#: Six distinct ability modifiers (+1, 0, -1, +4, +2, +3) so a rule that reads the wrong
#: ability shows up as a different number on both sides.
SCORES = {"STR": 12, "DEX": 10, "CON": 8, "INT": 18, "WIS": 14, "CHA": 16}
ABILITY_TOKEN = {
    "STR": "strength",
    "DEX": "dexterity",
    "CON": "constitution",
    "INT": "intelligence",
    "WIS": "wisdom",
    "CHA": "charisma",
}
#: (engine class token, PCGen CLASS name) -- the eleven Core Rulebook classes the chassis holds.
CLASSES = [
    ("barbarian", "Barbarian"),
    ("bard", "Bard"),
    ("cleric", "Cleric"),
    ("druid", "Druid"),
    ("fighter", "Fighter"),
    ("monk", "Monk"),
    ("paladin", "Paladin"),
    ("ranger", "Ranger"),
    ("rogue", "Rogue"),
    ("sorcerer", "Sorcerer"),
    ("wizard", "Wizard"),
]
LEVELS = [1, 10]
#: (engine race token, PCGen RACE name) -- the seven Core Rulebook races the chassis holds.
RACES = [
    ("human", "Human"),
    ("dwarf", "Dwarf"),
    ("elf", "Elf"),
    ("gnome", "Gnome"),
    ("half-elf", "Half-Elf"),
    ("half-orc", "Half-Orc"),
    ("halfling", "Halfling"),
]

PCG_TEMPLATE = """PCGVERSION:2.0

# System Information
CAMPAIGN:Core Rulebook
VERSION:6.09.08.RC1
GAMEMODE:Pathfinder_RPG
CHARACTERTYPE:PC
PURCHASEPOINTS:N
AUTOSPELLS:Y

# Character Bio
CHARACTERNAME:{name}
PLAYERNAME:sd35-at-35-e2-005

# Character Attributes
STAT:STR|SCORE:{STR}
STAT:DEX|SCORE:{DEX}
STAT:CON|SCORE:{CON}
STAT:INT|SCORE:{INT}
STAT:WIS|SCORE:{WIS}
STAT:CHA|SCORE:{CHA}
ALIGN:LN
RACE:{race}

# Character Class(es)
CLASS:{pcgen_class}|LEVEL:{level}|SKILLPOOL:0

# Character Experience
EXPERIENCE:0
EXPERIENCETABLE:Medium
{extra}"""

HUMAN_BONUS_PCG = "ABILITY:Ability Bonus|TYPE:NORMAL|CATEGORY:Special Ability|KEY:+2 Strength|TYPE:AbilityBonus\n"


def roster_members():
    """`(name, race token, PCGen race, class token, PCGen class, level)` for the generated
    members. The deterministic fighter is copied, not generated."""
    out = []
    for cls, pcls in CLASSES:
        for level in LEVELS:
            out.append((f"human_{cls}_l{level}", "human", "Human", cls, pcls, level))
    for race, prace in RACES:
        if race == "human":
            continue
        out.append((f"{race.replace('-', '_')}_fighter_l1", race, prace, "fighter", "Fighter", 1))
    return out


def engine_fixture_text(name, race, cls, level):
    lines = [
        f"# SD-35 AT-35-E2-005 parity roster member {name} (generated by sheet_parity.py roster).",
        f"case_id=parity-{name}",
        "source_package_id=pf1.core_rulebook",
        f"race_id=race:{race}",
        f"class_level=class:{cls}:{level}",
    ]
    for k, token in ABILITY_TOKEN.items():
        lines.append(f"ability={token}:{SCORES[k]}")
    if race == "human":
        lines.append("choice=choice:human_ability_bonus:ability:strength")
    return "\n".join(lines) + "\n"


def pcg_text(name, prace, pcls, level):
    extra = HUMAN_BONUS_PCG if prace == "Human" else ""
    return PCG_TEMPLATE.format(name=name, race=prace, pcgen_class=pcls, level=level, extra=extra, **SCORES)


def cmd_roster(args):
    os.makedirs(args.out, exist_ok=True)
    n = 0
    for name, race, prace, cls, pcls, level in roster_members():
        with open(os.path.join(args.out, f"{name}.txt"), "w", encoding="utf-8") as f:
            f.write(engine_fixture_text(name, race, cls, level))
        with open(os.path.join(args.out, f"{name}.pcg"), "w", encoding="utf-8") as f:
            f.write(pcg_text(name, prace, pcls, level))
        n += 1
    shutil.copyfile(DETERMINISTIC_FIXTURE, os.path.join(args.out, "deterministic_human_fighter_l1.txt"))
    shutil.copyfile(DETERMINISTIC_PCG, os.path.join(args.out, "deterministic_human_fighter_l1.pcg"))
    n += 1
    print(f"sheet_parity roster: members={n} -> {args.out}")
    return 0


# ---------------------------------------------------------------------------
# PCGen export
# ---------------------------------------------------------------------------


def run_one(pcg, out_txt, settings):
    log = out_txt + ".log"
    with open(log, "w", encoding="utf-8") as lf:
        proc = subprocess.run(["bash", RUN_ONE, pcg, FTL, out_txt, settings], stdout=lf, stderr=subprocess.STDOUT)
    return proc.returncode


def cmd_export(args):
    os.makedirs(args.out, exist_ok=True)
    pcgs = sorted(p for p in os.listdir(args.roster) if p.endswith(".pcg"))
    settings_root = os.path.join(args.out, "_settings")
    jobs = []
    for p in pcgs:
        name = p[: -len(".pcg")]
        jobs.append((os.path.join(args.roster, p), os.path.join(args.out, f"{name}.txt"), os.path.join(settings_root, name)))
    failures = 0
    with ThreadPoolExecutor(max_workers=args.jobs) as pool:
        for (pcg, out_txt, _), rc in zip(jobs, pool.map(lambda j: run_one(*j), jobs)):
            ok = rc == 0 and os.path.exists(out_txt)
            print(f"  {'ok  ' if ok else 'FAIL'} {os.path.basename(pcg)} exit={rc}")
            failures += 0 if ok else 1
    print(f"sheet_parity export: characters={len(jobs)} failed={failures} -> {args.out}")
    return 1 if failures else 0


# ---------------------------------------------------------------------------
# The comparison
# ---------------------------------------------------------------------------


def parse_export(text):
    """`KEY=VALUE` lines; a line without `=` continues the previous value (a multi-paragraph
    DESCRIPTION). Blank lines and `#` comments are skipped."""
    out = {}
    last = None
    for raw in text.splitlines():
        line = raw.rstrip()
        if not line.strip() or line.startswith("#"):
            continue
        m = re.match(r"^([A-Z][A-Z0-9_.]*)=(.*)$", line)
        if m:
            last = m.group(1)
            out[last] = m.group(2).strip()
        elif last is not None:
            out[last] = out[last] + "\n" + line.strip()
    return out


def as_int(s):
    if s is None:
        return None
    s = str(s).strip()
    m = re.match(r"^[+-]?\d+", s)
    return int(m.group(0)) if m else None


def ints_in(text):
    return {int(x) for x in re.findall(r"(?<![\w/])[+-]?\d+(?![\w])", text or "")}


def oracle_sha():
    try:
        with open(PIN_FILE, encoding="utf-8") as f:
            for line in f:
                if line.startswith("PCGEN_ORACLE_SHA="):
                    return line.split("=", 1)[1].strip()
    except OSError:
        pass
    return ""


#: Pathfinder: typed bonuses of these types stack; every other same-type pair takes the max
#: (mirrors `rules_core::sheet_rule::STACKING_TYPES`).
STACKING_TYPES = {"Defense", "Dodge", "Circumstance", "Racial", "NotRanged", "NotFlatFooted"}


def fold(contributions):
    """`[(value, bonus_type or None)]` -> one number, PCGen's `BonusManager` fold: untyped,
    stacking-typed, `Stack`-mode and negative values sum; a same-type pair otherwise takes the
    max."""
    total = 0
    best = {}
    for value, btype in contributions:
        name = (btype or {}).get("name") if btype else None
        mode = (btype or {}).get("mode") if btype else None
        if not name or mode == "Stack" or value < 0 or name in STACKING_TYPES:
            total += value
        else:
            best[name] = max(best.get(name, value), value)
    return total + sum(best.values())


def skill_name(slug):
    """`knowledge_arcana` -> `Knowledge (Arcana)`; `sleight_of_hand` -> `Sleight of Hand`."""
    for head in ("knowledge", "craft", "perform", "profession"):
        if slug.startswith(head + "_"):
            sub = slug[len(head) + 1 :].replace("_", " ").title()
            return f"{head.title()} ({sub})"
    words = slug.replace("_", " ").split()
    return " ".join(w if w in ("of",) else w.title() for w in words)


def index_by_name(export, prefix):
    """`SKILL.3.NAME=Climb` -> {"climb": "3"} for `prefix="SKILL"`."""
    out = {}
    for k, v in export.items():
        m = re.match(rf"^{prefix}\.(\d+)\.NAME$", k)
        if m:
            out[v.strip().lower()] = m.group(1)
    return out


def ability_names(export):
    """`STAT.0.NAME=STR` -> {"Str": "0"}."""
    out = {}
    for k, v in export.items():
        m = re.match(r"^STAT\.(\d+)\.NAME$", k)
        if m:
            out[v.strip().title()] = m.group(1)
    return out


#: PCGen's AC parts by bonus type (`AC.Armor`, `AC.Dodge`, ... in the stock sheet); an
#: untyped or other-typed AC contribution is PCGen's `AC.Misc`.
AC_PART_BY_TYPE = {
    "Armor": "AC.ARMOR",
    "Shield": "AC.SHIELD",
    "Dodge": "AC.DODGE",
    "NaturalArmor": "AC.NATURALARMOR",
    "Natural": "AC.NATURALARMOR",
    "Deflection": "AC.DEFLECTION",
    "Size": "AC.SIZE",
    "Ability": "AC.ABILITY",
}

SAVE_INDEX = {"Fortitude": "0", "Reflex": "1", "Will": "2"}
SAVE_ABILITY = {"Fortitude": "Con", "Reflex": "Dex", "Will": "Wis"}


def target_kind(target):
    """`{"Skill": "climb"}` -> ("Skill", "climb"); `"Ac"` -> ("Ac", None)."""
    if target is None:
        return (None, None)
    if isinstance(target, str):
        return (target, None)
    if isinstance(target, dict) and len(target) == 1:
        k, v = next(iter(target.items()))
        return (k, v)
    return ("?", target)


def oracle_component(export, kind, arg, character):
    """The PCGen component a folded contribution on `(kind, arg)` compares with, as
    `(value, key)`; `(None, reason)` when PCGen exports no component for it."""
    stats = ability_names(export)
    if kind == "Skill":
        idx = index_by_name(export, "SKILL").get(skill_name(arg).lower())
        if idx is None:
            return (None, f"no-pcgen-skill-row:{skill_name(arg)}")
        return (as_int(export.get(f"SKILL.{idx}.MISC")), f"SKILL.{idx}.MISC")
    if kind == "Ac":
        # `arg` is the bonus type the contributions were folded under (see `compare_character`);
        # PCGen exports AC by type, so each type compares with its own part.
        key = AC_PART_BY_TYPE.get(arg or "", "AC.MISC")
        v = as_int(export.get(key))
        return (v, key) if v is not None else (None, f"no-pcgen-ac-part:{key}")
    if kind == "Save":
        i = SAVE_INDEX.get(arg)
        misc = as_int(export.get(f"CHECK.{i}.MISC"))
        abil = as_int(export.get(f"STAT.{stats.get(SAVE_ABILITY[arg], 'x')}.MOD"))
        if misc is None or abil is None:
            return (None, f"no-pcgen-check-misc:{arg}")
        return (misc - abil, f"CHECK.{i}.MISC-STAT.{SAVE_ABILITY[arg].upper()}.MOD")
    if kind == "BaseSave":
        i = SAVE_INDEX.get(arg)
        return (as_int(export.get(f"CHECK.{i}.BASE")), f"CHECK.{i}.BASE")
    if kind == "BaseAttack":
        return (as_int(export.get("BAB")), "BAB")
    if kind == "Initiative":
        return (as_int(export.get("INITIATIVEMISC")), "INITIATIVEMISC")
    if kind == "Attack":
        return (as_int(export.get("ATTACK.MELEE.MISC")), "ATTACK.MELEE.MISC")
    if kind == "Dr":
        v = as_int(export.get("DR"))
        return (v, "DR") if v is not None else (None, "no-pcgen-dr")
    if kind == "Speed":
        idx = index_by_name(export, "MOVE").get(str(arg).lower())
        if idx is None:
            return (None, f"no-pcgen-move-row:{arg}")
        base = (character.get("facts", {}).get("speeds") or {}).get(arg)
        if base is None:
            return (None, f"no-engine-base-speed:{arg}")
        return (as_int(export.get(f"MOVE.{idx}.RATE")) - base, f"MOVE.{idx}.RATE-{base}")
    if kind == "Ability":
        i = stats.get(arg)
        if i is None:
            return (None, f"no-pcgen-stat:{arg}")
        order = ["Str", "Dex", "Con", "Int", "Wis", "Cha"]
        base = character["ability_scores"][order.index(arg)]
        return (as_int(export.get(f"STAT.{i}.SCORE")) - base, f"STAT.{i}.SCORE-{base}")
    return (None, f"no-component-export:{kind}")


def named_abilities(export):
    """`{lower name: [desc, ...]}` over SA.n and FEAT.n."""
    out = {}
    for prefix in ("SA", "FEAT"):
        for k, v in export.items():
            m = re.match(rf"^{prefix}\.(\d+)\.NAME$", k)
            if m:
                out.setdefault(v.strip().lower(), []).append(export.get(f"{prefix}.{m.group(1)}.DESC", ""))
    return out


def label_keys(label):
    """The names to try for a line's label: as printed, without a parenthetical, the part
    after ` ~ `."""
    keys = [label]
    if "(" in label:
        keys.append(label.split("(", 1)[0])
    if " ~ " in label:
        keys.append(label.split(" ~ ", 1)[1])
    return [k.strip().lower() for k in keys if k.strip()]


def rule_expr(rule_id):
    """The converted rule's `value` (its `Expr`) from `data/sheet_rules/`, for naming a
    disagreement."""
    book, kind, slug = rule_id.split(":", 2)
    slug = slug.split("#", 1)[0]
    path = os.path.join(REPO, "data", "sheet_rules", book, kind, f"{slug}.json")
    try:
        with open(path, encoding="utf-8") as f:
            rules = json.load(f)
    except (OSError, ValueError):
        return None
    for r in rules:
        if r.get("id") == rule_id:
            return {"value": r.get("value"), "also": r.get("also")}
    return {"value": rules[0].get("value"), "also": rules[0].get("also")} if rules else None


def line_number(value):
    if isinstance(value, dict) and "Resolved" in value:
        return value["Resolved"]
    return None


def compare_character(character, export):
    """One roster member -> the result rows."""
    name = character["character"]
    rows = []

    def row(family, unit, ours, oracle, key, verdict, **extra):
        r = {"character": name, "family": family, "unit": unit, "ours": ours, "oracle": oracle, "oracle_key": key, "verdict": verdict}
        r.update(extra)
        rows.append(r)

    # chassis totals
    stats = ability_names(export)
    order = ["Str", "Dex", "Con", "Int", "Wis", "Cha"]
    for i, a in enumerate(order):
        key = f"STAT.{stats.get(a, 'x')}.MOD"
        o = as_int(export.get(key))
        v = character["chassis"]["ability_mods"][i]
        row("chassis", f"ability_mod.{a.lower()}", v, o, key, "unverifiable" if o is None else ("agree" if o == v else "disagree"))
    o = as_int(export.get("BAB"))
    row("chassis", "base_attack_bonus", character["chassis"]["bab"], o, "BAB", "unverifiable" if o is None else ("agree" if o == character["chassis"]["bab"] else "disagree"))
    for i, s in enumerate(["Fortitude", "Reflex", "Will"]):
        for part, ours_vals in (("BASE", character["chassis"]["base_saves"]), ("TOTAL", character["chassis"]["total_saves"])):
            key = f"CHECK.{SAVE_INDEX[s]}.{part}"
            o = as_int(export.get(key))
            v = ours_vals[i]
            row("chassis", f"save.{s.lower()}.{part.lower()}", v, o, key, "unverifiable" if o is None else ("agree" if o == v else "disagree"))
    # `baseline_armor_class`, `baseline_melee_attack_bonus` and the selected skill modifiers
    # are documented as ZERO when the deterministic Chain Shirt / Longsword / Climb-Intimidate-
    # Swim posture is absent (`pilot_compute::PilotBaseChassisComputation`); that zero is a
    # sentinel, not a total, so it is `unverifiable` here rather than a disagreement.
    # The baseline melee attack is the wielded longsword's line (weapon-specific bonuses such
    # as Weapon Focus included), so it compares with `WEAPON.0.TOTALHIT` when a weapon is
    # wielded and with the generic `ATTACK.MELEE.TOTAL` otherwise.
    melee_key = "WEAPON.0.TOTALHIT" if as_int(export.get("WEAPON.0.TOTALHIT")) is not None else "ATTACK.MELEE.TOTAL"
    for unit, key, v in (("armor_class", "AC.TOTAL", character["chassis"]["baseline_ac"]), ("melee_attack", melee_key, character["chassis"]["baseline_melee_attack"])):
        o = as_int(export.get(key))
        if v == 0:
            row("chassis", unit, v, o, "engine-posture-absent", "unverifiable")
            continue
        row("chassis", unit, v, o, key, "unverifiable" if o is None else ("agree" if o == v else "disagree"))
    skill_idx = index_by_name(export, "SKILL")
    skills = character["chassis"]["skills"]
    for slug, v in skills.items():
        idx = skill_idx.get(skill_name(slug).lower())
        key = f"SKILL.{idx}.TOTAL" if idx else f"SKILL.{skill_name(slug)}.TOTAL"
        o = as_int(export.get(key)) if idx else None
        if all(x == 0 for x in skills.values()):
            row("chassis", f"skill.{slug}", v, o, "engine-posture-absent", "unverifiable")
            continue
        row("chassis", f"skill.{slug}", v, o, key, "unverifiable" if o is None else ("agree" if o == v else "disagree"))

    # lines: fold the targeted Number lines per target
    by_target = {}
    standalone = []
    for line in character["lines"]:
        n = line_number(line.get("value"))
        kind, arg = target_kind(line.get("target"))
        if n is not None and kind is not None:
            if kind == "Ac":
                # AC folds per bonus type: PCGen exports one part per type.
                btype = (line.get("bonus_type") or {}).get("name")
                arg = btype if btype in AC_PART_BY_TYPE else None
            by_target.setdefault((kind, json.dumps(arg, sort_keys=True)), []).append(line)
        elif n is not None:
            standalone.append((line, n, "value"))
        for role, also_value in line.get("also") or []:
            an = line_number(also_value)
            if an is not None:
                standalone.append((line, an, f"also:{role}"))
    for (kind, arg_json), lines in sorted(by_target.items()):
        arg = json.loads(arg_json)
        ours = fold([(line_number(l["value"]), l.get("bonus_type")) for l in lines])
        oracle, key = oracle_component(export, kind, arg, character)
        ids = [l["id"] for l in lines]
        if oracle is None:
            verdict = "unverifiable"
        else:
            verdict = "agree" if oracle == ours else "disagree"
        extra = {"rule_ids": ids, "labels": [l["label"] for l in lines], "contributions": [line_number(l["value"]) for l in lines]}
        if verdict == "disagree":
            extra["expr"] = {i: rule_expr(i) for i in ids}
        unit = f"target:{kind}" if arg is None else f"target:{kind}:{arg if isinstance(arg, str) else json.dumps(arg)}"
        row("lines", unit, ours, oracle, key, verdict, **extra)

    # lines: standalone numbers against the same-named ability's substituted description
    abilities = named_abilities(export)
    for line, n, slot in standalone:
        descs = None
        matched = None
        for k in label_keys(line["label"]):
            if k in abilities:
                descs = abilities[k]
                matched = k
                break
        if descs is None:
            row("lines", f"{line['id']}:{slot}", n, None, "no-pcgen-ability-named:" + line["label"], "unverifiable", rule_ids=[line["id"]], labels=[line["label"]])
            continue
        found = set()
        for d in descs:
            found |= ints_in(d)
        if not found:
            row("lines", f"{line['id']}:{slot}", n, None, f"DESC-has-no-number:{matched}", "unverifiable", rule_ids=[line["id"]], labels=[line["label"]])
            continue
        verdict = "agree" if n in found else "disagree"
        extra = {"rule_ids": [line["id"]], "labels": [line["label"]], "desc_numbers": sorted(found)}
        if verdict == "disagree":
            extra["expr"] = {line["id"]: rule_expr(line["id"])}
            extra["desc"] = descs[0][:400]
        row("lines", f"{line['id']}:{slot}", n, sorted(found), f"DESC:{matched}", verdict, **extra)
    return rows


def summarize(rows):
    out = {}
    for fam in ("lines", "chassis"):
        c = {"compared": 0, "agree": 0, "disagree": 0, "unverifiable": 0}
        for r in rows:
            if r["family"] != fam:
                continue
            c[r["verdict"]] += 1
            if r["verdict"] in ("agree", "disagree"):
                c["compared"] += 1
        out[fam] = c
    reasons = {}
    for r in rows:
        if r["verdict"] == "unverifiable":
            reason = r["oracle_key"].split(":", 1)[0] if r["family"] == "lines" else r["oracle_key"]
            reasons[reason] = reasons.get(reason, 0) + 1
    out["unverifiable_reasons"] = dict(sorted(reasons.items(), key=lambda kv: (-kv[1], kv[0])))
    return out


def cmd_compare(args):
    with open(args.ours, encoding="utf-8") as f:
        ours = json.load(f)
    rows = []
    missing = []
    for character in ours["characters"]:
        path = os.path.join(args.exports, character["character"] + ".txt")
        if not os.path.exists(path):
            missing.append(character["character"])
            continue
        with open(path, encoding="utf-8") as f:
            export = parse_export(f.read())
        rows.extend(compare_character(character, export))
    summary = summarize(rows)
    sha = oracle_sha()
    doc = {
        "pcgen_oracle_sha": sha,
        "roster": [c["character"] for c in ours["characters"]],
        "exports_missing": missing,
        "summary": summary,
        "disagreements": [r for r in rows if r["verdict"] == "disagree"],
        "results": rows,
    }
    with open(args.output, "w", encoding="utf-8") as f:
        json.dump(doc, f, indent=2, sort_keys=False)
        f.write("\n")
    ln = summary["lines"]
    ch = summary["chassis"]
    print(
        f"sheet_parity: lines compared={ln['compared']} agree={ln['agree']} disagree={ln['disagree']} "
        f"unverifiable={ln['unverifiable']}; chassis compared={ch['compared']} agree={ch['agree']} "
        f"disagree={ch['disagree']} unverifiable={ch['unverifiable']}; characters={len(ours['characters']) - len(missing)} "
        f"exports_missing={len(missing)} PCGEN_ORACLE_SHA={sha} -> {args.output}"
    )
    for r in doc["disagreements"]:
        print(f"  DISAGREE {r['character']} {r['unit']} ours={r['ours']} oracle={r['oracle']} ({r['oracle_key']})")
    return 0


def main(argv=None):
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = p.add_subparsers(dest="cmd", required=True)
    r = sub.add_parser("roster")
    r.add_argument("--out", required=True)
    r.set_defaults(fn=cmd_roster)
    e = sub.add_parser("export")
    e.add_argument("--roster", required=True)
    e.add_argument("--out", required=True)
    e.add_argument("--jobs", type=int, default=3)
    e.set_defaults(fn=cmd_export)
    c = sub.add_parser("compare")
    c.add_argument("--ours", required=True)
    c.add_argument("--exports", required=True)
    c.add_argument("--output", required=True)
    c.set_defaults(fn=cmd_compare)
    args = p.parse_args(argv)
    return args.fn(args)


if __name__ == "__main__":
    sys.exit(main())
