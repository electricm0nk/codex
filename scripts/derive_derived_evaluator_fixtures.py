#!/usr/bin/env python3
"""Derive the `derived` evaluator-vs-fixture expectations FROM THE CORPUS.

This is the documented derivation behind `tests/fixtures/rules_core/
derived-evaluator-fixtures.json`, the fixture the evaluator-vs-fixture check
(`tests/derived_evaluator_fixture_check.rs`) tests the engine against.

WHY THE PROVENANCE RULE MATTERS
-------------------------------
The former `SD-32-instrument-coverage-and-consumer-wiring/epic-breakdown.md` E6-F1 (absorbed into
`docs/release/SD-30-class-feature-archetype-bundle/`, package deleted per `decisions.md §50`; text
retrievable from git history at commit `b88b18fa3700125f992e67b0ae29e1d5b70de3c0`):
"A fixture generated from the evaluator's own output is worthless."
This repo has a live finding of exactly that shape (SD-29 Epic 10 F3 /
register C1.4a): frontend preview fixtures were hand-authored rules data with
nothing pinning them to the corpus, and one shipped wrong rules content to a
player.

So this script is constrained by construction:

* It reads the UPSTREAM PCGen `.lst` bytes (`$PCGEN_CORPUS_ROOT`, default
  `$HOME/workspace/repos/pcgen/data`) and nothing else for every VALUE it
  emits. No engine module is imported; no engine binary is run; no file under
  `data/corpus/` (this repo's ingested JSON, which is what the engine actually
  reads) is opened.
* It reads `docs/work-inventory.json` for IDENTITY ONLY -- which units are
  `derived` and held, and which `.lst` file and line each one came from.
  No magnitude, no status-derived number, and no engine-computed value is
  copied out of it. That file is engine-produced, so using any VALUE from it
  would break the independence this script exists to guarantee.

The two sources the check compares are therefore genuinely different artifacts
on genuinely different trees: the fixture is anchored to the upstream PCGen
`.lst`, while the engine evaluates this repo's own `data/corpus/**.json`
ingest of the same rows. A drift between the two makes the check RED, which
is the whole point -- `docs/retro` memory "generated artifacts mutated
post-hoc" is a recorded hazard with no test covering it until now.

WHAT `derived` MEANS, AND WHAT ITS BAR IS
-----------------------------------------
`src/rules_core/wiring_class.rs`: a row is `derived` when a magnitude-bearing
field's value is a function of a character/item scalar rather than a bare
literal. The dashboard's own `doneness_meaning` states the bar for a `derived`
unit: "needs an evaluator-vs-fixture check" -- the engine's evaluator output
must equal an independently stated expected value.

TOKEN FAMILY COVERED
--------------------
`BONUS:STAT|<ability list>|<value>[|<extra>...]` -- PCGen's ability-score
bonus token. Chosen because it is the only derived-bearing token family in the
corpus for which this engine has a real evaluator that produces a NUMBER
(`rules_core::equipment_effects::magic_items::compute_magic_items_effect`,
reached through the shipping `compute_equipment_effects` seam). Every other
`derived` token family found on held units is reported by `--report` with the
reason it is not covered; leaving a family uncovered is an honest outcome and
is recorded rather than papered over.

Derivation rule for this family, from PCGen's token grammar:

    BONUS:STAT|STR|2|TYPE=Enhancement
              ^^^ ^ ^^^^^^^^^^^^^^^^
              |   |  stacking type, does not change the magnitude
              |   value: the integer added to each named ability
              comma-separated ability list

    expected := { abilities: ["STR"], bonus: 2 }

A row carrying several `BONUS:STAT` fields contributes its FIRST one, which is
the field PCGen itself applies first and the field the engine's evaluator
reads; rows like that are flagged `multi_stat_row` in the fixture so the check
can be widened deliberately rather than by accident.

RUN
---
    python3 scripts/derive_derived_evaluator_fixtures.py            # write
    python3 scripts/derive_derived_evaluator_fixtures.py --report   # survey only
"""

import argparse
import collections
import hashlib
import json
import os
import re
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
INVENTORY = os.path.join(REPO, "docs", "work-inventory.json")
FIXTURE = os.path.join(
    REPO, "tests", "fixtures", "rules_core", "derived-evaluator-fixtures.json"
)
CORPUS_RELATIVE_ROOT = "pathfinder/paizo/roleplaying_game"

# Transcribed from `src/rules_core/wiring_class.rs` (MAGNITUDE_TOKENS,
# SCALARS_SUBSTRING, SCALARS_WORD, has_arith). Used ONLY to find which field of
# a row carries the derived magnitude -- never to decide a unit's class, which
# `docs/work-inventory.json` already carries. Reproducing the field-level scan
# here is what lets the fixture name the exact corpus field it was derived from.
MAGNITUDE_TOKENS = (
    "BONUS:", "TEMPBONUS:", "DEFINE:", "COST:", "WT:", "CR:", "AC:", "ACCHECK:",
    "DAMAGE:", "CRITMULT:", "CRITRANGE:", "RANGE:", "REACH:", "MOVE:", "HITDIE:",
    "LEVELADJUSTMENT:", "SR:", "DR:", "SPELLFAILURE:", "STAT:",
)
SCALARS_SUBSTRING = ("CASTERLEVEL", "CLASSLEVEL", "TOTALLEVELS", "TOTALLEVEL",
                     "PLUSTOTAL", "SPELLLEVEL")
SCALARS_WORD = ("BAB", "HD", "STR", "DEX", "CON", "INT", "WIS", "CHA", "TL",
                "CL", "RACESIZE")
HELD_STATUSES = ("ingested-magnitude", "grounded", "text-complete")

# The engine reads its equipment records from `data/corpus/<book>/equipment/`.
# Only `equipment`-family kinds reach `compute_equipment_effects` at all.
EVALUATED_KINDS = ("equipment", "equipment_modifier")


def pcgen_data_root():
    root = os.environ.get("PCGEN_CORPUS_ROOT")
    if root:
        return root
    home = os.environ.get("HOME")
    if not home:
        sys.exit("HOME must be set (or PCGEN_CORPUS_ROOT given) to locate the PCGen corpus")
    return os.path.join(home, "workspace", "repos", "pcgen", "data")


def has_scalar(value):
    if any(s in value for s in SCALARS_SUBSTRING):
        return True
    return any(
        re.search(r"(?<![A-Za-z0-9_])" + re.escape(s) + r"(?![A-Za-z0-9_])", value)
        for s in SCALARS_WORD
    )


def has_arith(value):
    if "*" in value or "/" in value:
        return True
    if "min(" in value.lower() or "max(" in value.lower():
        return True
    return re.search(r"\+\s*\w*[A-Z]{2,}", value) is not None


def derived_fields(line):
    """The magnitude-bearing fields of one raw `.lst` row that are derived."""
    fields = [f.strip() for f in line.rstrip("\r\n").split("\t")]
    out = []
    for field in fields:
        if not any(field.startswith(t) for t in MAGNITUDE_TOKENS):
            continue
        value = field.split(":", 1)[1] if ":" in field else ""
        if has_scalar(value) or has_arith(value):
            out.append(field)
    return fields, out


def record_key_of(fields):
    """The row's corpus identity: its `KEY:` token, else its first column.

    Exactly what PCGen itself does, and exactly the identity
    `equipment_resolver::equipment_id_resolve` matches on.
    """
    for field in fields:
        if field.startswith("KEY:"):
            return field[len("KEY:"):]
    return fields[0] if fields else ""


def parse_bonus_stat(field):
    """`BONUS:STAT|<abilities>|<value>[|...]` -> (abilities, value) or None.

    The whole derivation. It is deliberately tiny and deliberately total: a
    field it cannot parse yields None and the unit is reported uncovered
    rather than given a guessed expectation.
    """
    if not field.startswith("BONUS:STAT|"):
        return None
    parts = field[len("BONUS:"):].split("|")
    if len(parts) < 3 or parts[0] != "STAT":
        return None
    abilities = [a.strip() for a in parts[1].split(",") if a.strip()]
    try:
        value = int(parts[2])
    except ValueError:
        return None
    if not abilities:
        return None
    return abilities, value


def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as fh:
        for chunk in iter(lambda: fh.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def index_corpus(root):
    """basename -> [absolute paths], over the PCGen roleplaying_game tree."""
    index = collections.defaultdict(list)
    for dirpath, _dirs, files in os.walk(root):
        for name in files:
            index[name].append(os.path.join(dirpath, name))
    return index


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--report", action="store_true",
                    help="print the coverage survey and write nothing")
    args = ap.parse_args()

    data_root = pcgen_data_root()
    corpus_root = os.path.join(data_root, CORPUS_RELATIVE_ROOT)
    if not os.path.isdir(corpus_root):
        sys.exit(f"PCGen corpus not found at {corpus_root}; set PCGEN_CORPUS_ROOT")

    with open(INVENTORY, encoding="utf-8") as fh:
        inventory = json.load(fh)

    index = index_corpus(corpus_root)
    sha_cache = {}
    text_cache = {}

    entries = []
    uncovered = collections.Counter()
    multi_stat = 0

    held_derived = [
        u for u in inventory["units"]
        if u.get("wiring_class") == "derived" and u["status"] in HELD_STATUSES
    ]

    for unit in held_derived:
        if unit["kind"] not in EVALUATED_KINDS:
            uncovered[f"kind={unit['kind']}: no equipment-effect evaluator"] += 1
            continue
        candidates = index.get(unit["source_file"], [])
        scoped = [p for p in candidates if f"/{unit['book']}/" in p] or candidates
        if not scoped:
            uncovered["source .lst not found under the PCGen corpus root"] += 1
            continue
        path = scoped[0]
        if path not in text_cache:
            with open(path, encoding="utf-8", errors="replace") as fh:
                text_cache[path] = fh.read().split("\n")
            sha_cache[path] = sha256_file(path)
        lines = text_cache[path]
        if unit["source_line"] < 1 or unit["source_line"] > len(lines):
            uncovered["cited source_line is outside the .lst"] += 1
            continue
        line = lines[unit["source_line"] - 1]
        fields, derived = derived_fields(line)
        stat_fields = [f for f in derived if f.startswith("BONUS:STAT|")]
        if not stat_fields:
            families = ",".join(sorted({f.split("|")[0] for f in derived})) or "(none)"
            uncovered[f"derived token family with no numeric evaluator: {families}"] += 1
            continue
        parsed = parse_bonus_stat(stat_fields[0])
        if parsed is None:
            uncovered["BONUS:STAT field the derivation cannot parse"] += 1
            continue
        abilities, value = parsed
        if len(stat_fields) > 1:
            multi_stat += 1
        entries.append({
            "unit_id": unit["id"],
            "book": unit["book"],
            "kind": unit["kind"],
            "record_key": record_key_of(fields),
            "upstream_lst": os.path.relpath(path, data_root),
            "upstream_lst_sha256": sha_cache[path],
            "upstream_line": unit["source_line"],
            "corpus_field": stat_fields[0],
            "token_family": "BONUS:STAT",
            "multi_stat_row": len(stat_fields) > 1,
            "expected": {"abilities": abilities, "bonus": value},
        })

    entries.sort(key=lambda e: e["unit_id"])

    print(f"held `derived` units in the inventory: {len(held_derived)}")
    print(f"fixture entries derived from the corpus: {len(entries)}")
    print(f"  of which rows carrying >1 BONUS:STAT field: {multi_stat}")
    print("uncovered, by reason:")
    for reason, n in uncovered.most_common():
        print(f"  {n:6}  {reason}")
    print(f"  {sum(uncovered.values()):6}  TOTAL uncovered")

    if args.report:
        return

    # SD31-E6-F11-002: this script only ever derived the `kind=equipment`
    # `BONUS:STAT` family below and rebuilds the WHOLE committed file from
    # scratch every run. A sibling `kind=monster` family
    # (`monster_entries`/`monster_token_family`/`monster_derivation`/
    # `monster_independence`, the `BONUS:VAR|SLA_CL|HD` universal-monster-rule
    # seam -- see `src/rules_core/derived_evaluator_fixture_check.rs` and
    # `progress.md`'s `SD31-E6-F11-002` receipt) is hand-derived and
    # hand-appended directly to the committed JSON, not generated by this
    # script. Preserve it across a re-run rather than silently dropping it --
    # this script's own job is `entries` (equipment) only, and a future
    # extension that teaches it the monster family should replace this
    # preserve-as-is block with a real generator, not remove the block.
    monster_keys = {}
    if os.path.exists(FIXTURE):
        with open(FIXTURE, encoding="utf-8") as fh:
            existing = json.load(fh)
        monster_keys = {k: v for k, v in existing.items() if k.startswith("monster_")}
        if monster_keys:
            print(
                f"preserving {len(monster_keys.get('monster_entries', []))} monster_entries "
                "row(s) from the existing fixture (this script does not generate them)"
            )

    document = {
        "schema": 1,
        "generated_by": "scripts/derive_derived_evaluator_fixtures.py",
        "pcgen_corpus_relative_root": CORPUS_RELATIVE_ROOT,
        "token_family": "BONUS:STAT",
        "derivation": (
            "PCGen `BONUS:STAT|<comma-separated ability list>|<integer value>"
            "[|<extra qualifiers>]`. `expected.abilities` is the ability list "
            "split on commas; `expected.bonus` is the integer value. Trailing "
            "qualifiers (`TYPE=Enhancement`, ...) select a stacking type and do "
            "not change the magnitude. Read straight out of the upstream PCGen "
            "`.lst` bytes named by `upstream_lst`/`upstream_line`."
        ),
        "independence": (
            "Every value here is a function of the upstream PCGen `.lst` bytes "
            "alone. The generator imports no engine module, runs no engine "
            "binary, and opens no file under `data/corpus/` -- which is the "
            "ingest the engine actually evaluates, and therefore the artifact "
            "this fixture must stay independent of. `docs/work-inventory.json` "
            "is read for unit identity and source-line provenance only. This "
            "fixture is committed before the check that consumes it is written "
            "(SD-32 E6-F1)."
        ),
        "entries": entries,
        **monster_keys,
    }
    os.makedirs(os.path.dirname(FIXTURE), exist_ok=True)
    with open(FIXTURE, "w", encoding="utf-8") as fh:
        json.dump(document, fh, indent=2, sort_keys=False)
        fh.write("\n")
    print(f"\nwrote {os.path.relpath(FIXTURE, REPO)}")


if __name__ == "__main__":
    main()
