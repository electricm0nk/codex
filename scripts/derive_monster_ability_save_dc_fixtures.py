#!/usr/bin/env python3
"""Derive the `kind=monster_ability` save-DC expectations FROM THE UPSTREAM CORPUS.

This is the documented derivation behind the `monster_ability_entries` array of
`tests/fixtures/rules_core/derived-evaluator-fixtures.json`, the fixture the
evaluator-vs-fixture check
(`tests/derived_evaluator_fixture_check_monster_ability.rs`) tests the engine
against. It is the sibling of `scripts/derive_derived_evaluator_fixtures.py`
(equipment `BONUS:STAT`) and `scripts/derive_spell_range_fixtures.py`, and it
keeps their independence rule verbatim:

* every VALUE it emits is a function of the upstream PCGen `.lst` bytes
  (`$PCGEN_CORPUS_ROOT`, pinned by `scripts/pcgen-oracle-pin.env`) alone. No
  engine module is imported, no engine binary is run, and **no file under
  `data/corpus/` is opened** -- that ingest is what the engine actually
  evaluates, so a value copied from it would make the check circular;
* `docs/work-inventory.json` is read for IDENTITY ONLY -- which units are
  `kind=monster_ability`, `wiring_class=derived`, `status=grounded` (the held
  population this seam exists to move), and which `.lst` file each one came
  from. No magnitude and no engine-computed value is copied out of it.

THE BAR, AND WHY IT IS NOT CIRCULAR
-----------------------------------
PF1's Universal Monster Rules state the save DC of a monster's special ability:

    "The save DC against a monster's special ability is equal to
     10 + 1/2 the monster's racial HD + the monster's relevant ability
     modifier."          (Bestiary, Appendix 1: Universal Monster Rules)

PCGen encodes the *result* of that formula on the ABILITY row, as the `DESC:`
token's `%N` argument, with the ability-modifier term left symbolic because it
depends on the creature's live ability score:

    DESC:...must succeed at a DC %1 Will save...|15+WIS
                                                 ^^ ^^^
                                                 |  the ability whose modifier is added
                                                 10 + 1/2 racial HD, already summed

and it encodes the creature's racial HD on a COMPLETELY DIFFERENT ROW, in a
different file, as the trailing segment of `MONSTERCLASS:<type>:<HD>`.

So this seam's expected value is fixed by TWO independent upstream facts that
the printed rule ties together:

    expected.save_dc_base   read off the ability row's DESC argument
    10 + racial_hd // 2     computed from the owning monster row's MONSTERCLASS

and an entry is emitted ONLY when they agree. Where they disagree we do not
know which side is right, so the unit is NOT fixtured and NOT credited -- it is
reported by `--report` instead. That predicate is stated before the run, not
chosen after it, and the disagreements are published rather than dropped.

The engine's own evaluator
(`rules_core::derived_evaluator_fixture_check::monster_ability_save_dc`) reads
neither of those upstream files: it parses the compiled
`monster_chassis::MONSTER_BOOKS` tables, generated from `data/corpus/`. Three
artifacts, three code paths.

THE LINKED-ABILITY REQUIREMENT
------------------------------
PCGen namespaces a monster's own ability rows as `<Monster> ~ <Ability>`. This
script resolves the owner by splitting that key and finding a row whose `KEY:`
is exactly `<Monster>` **in the same book directory**. An ability row that
resolves to no monster row in its own book is an ORPHAN -- a template-namespaced
row no monster applies -- and is excluded, because there is no racial HD to
apply the printed rule to. That exclusion is reported, never silently dropped.

RUN
---
    python3 scripts/derive_monster_ability_save_dc_fixtures.py            # write
    python3 scripts/derive_monster_ability_save_dc_fixtures.py --report   # survey only
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

# The six PF1 ability abbreviations, exactly as PCGen spells them in a formula.
# A bare abbreviation is the MODIFIER (PCGen spells the score `<ABBREV>SCORE`,
# which this corpus also uses -- `CHASCORE` appears on one row -- so the two are
# distinguishable and only the modifier form is accepted here).
STATS = ("STR", "DEX", "CON", "INT", "WIS", "CHA")
_S = "|".join(STATS)
# `<int>+<STAT>` or `<STAT>+<int>`; both orders occur in the corpus.
SAVE_DC_EXPR = re.compile(
    r"^\s*(?:(?P<c1>\d+)\s*\+\s*(?P<s1>%s)|(?P<s2>%s)\s*\+\s*(?P<c2>\d+))\s*$" % (_S, _S)
)
# A `%N` placeholder introduced by the literal word "DC" -- the only slot this
# seam claims. `DESC:...deals 3d8+%1 points...` is a damage term, not a save DC,
# and is excluded by this regex rather than by a hand-kept list.
DC_SLOT = re.compile(r"\bDC\s+%(\d+)")

TARGET_KIND = "monster_ability"
TARGET_WIRING = "derived"
TARGET_STATUS = "grounded"


def pcgen_data_root():
    root = os.environ.get("PCGEN_CORPUS_ROOT")
    if root:
        return root
    home = os.environ.get("HOME")
    if not home:
        sys.exit("HOME must be set (or PCGEN_CORPUS_ROOT given) to locate the PCGen corpus")
    return os.path.join(home, "workspace", "repos", "pcgen", "data")


def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as fh:
        for chunk in iter(lambda: fh.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def index_lst(root):
    """basename -> [absolute paths], over the whole pinned PCGen pathfinder tree."""
    index = collections.defaultdict(list)
    for dirpath, _dirs, files in os.walk(os.path.join(root, "pathfinder")):
        for name in files:
            if name.endswith(".lst"):
                index[name].append(os.path.join(dirpath, name))
    return index


def read_lines(path, cache):
    if path not in cache:
        with open(path, encoding="utf-8", errors="replace") as fh:
            cache[path] = fh.read().splitlines()
    return cache[path]


def fields_of(line):
    return [f.strip() for f in line.rstrip("\r\n").split("\t") if f.strip()]


def token(fields, name):
    prefix = name + ":"
    for f in fields:
        if f.startswith(prefix):
            return f[len(prefix):]
    return None


def parse_desc(raw):
    """`DESC:` value -> (prose, [args]). PCGen delimits args with `|`."""
    parts = raw.split("|")
    return parts[0], parts[1:]


def save_dc_slot(prose, args):
    """The `(slot_index, expression, constant, ability)` this row states, or None."""
    for m in DC_SLOT.finditer(prose):
        n = int(m.group(1))
        if n - 1 >= len(args):
            continue
        expr = args[n - 1]
        em = SAVE_DC_EXPR.match(expr)
        if not em:
            continue
        const = int(em.group("c1") or em.group("c2"))
        stat = em.group("s1") or em.group("s2")
        return n, expr, const, stat
    return None


def racial_hd(monsterclass_token):
    """`MONSTERCLASS:<type>:<HD>` -> HD, or None when the tail is not an integer."""
    if not monsterclass_token:
        return None
    tail = monsterclass_token.rsplit(":", 1)[-1].strip()
    try:
        return int(tail)
    except ValueError:
        return None


def find_owner_row(book_dir, owner_key, cache):
    """The `(path, line_no, fields)` of the monster row whose `KEY:` is `owner_key`.

    Searched in the ability file's OWN book directory (and its `support/`
    subdirectory, which Inner Sea Gods uses) -- never book-wide, because an
    ability owned by a monster in a different book is not the linked shape this
    seam credits.
    """
    hits = []
    for dirpath, _dirs, files in os.walk(book_dir):
        for name in sorted(files):
            if not name.endswith(".lst"):
                continue
            path = os.path.join(dirpath, name)
            for i, line in enumerate(read_lines(path, cache), start=1):
                if not line.strip() or line.lstrip().startswith("#"):
                    continue
                if ("KEY:" + owner_key) not in line:
                    continue
                fields = fields_of(line)
                if token(fields, "KEY") != owner_key:
                    continue
                if token(fields, "MONSTERCLASS") is None:
                    continue
                hits.append((path, i, fields))
    return hits


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--report", action="store_true", help="survey only; write nothing")
    args = ap.parse_args()

    root = pcgen_data_root()
    index = index_lst(root)
    cache = {}

    with open(INVENTORY, encoding="utf-8") as fh:
        inventory = json.load(fh)
    units = [
        u for u in inventory["units"]
        if u.get("kind") == TARGET_KIND
        and u.get("wiring_class") == TARGET_WIRING
        and u.get("status") == TARGET_STATUS
    ]

    buckets = collections.Counter()
    entries = []
    disagreements = []

    for unit in sorted(units, key=lambda u: u["id"]):
        src = unit.get("source_file")
        line_no = unit.get("source_line")
        paths = index.get(src or "", [])
        if len(paths) != 1:
            buckets["lst_path_ambiguous_or_absent"] += 1
            continue
        path = paths[0]
        lines = read_lines(path, cache)
        if not line_no or line_no > len(lines):
            buckets["line_out_of_range"] += 1
            continue
        fields = fields_of(lines[line_no - 1])
        key = token(fields, "KEY") or (fields[0] if fields else "")
        if key != unit.get("corpus_key"):
            buckets["key_mismatch_at_cited_line"] += 1
            continue
        desc_raw = token(fields, "DESC")
        if desc_raw is None:
            buckets["row_states_no_DESC"] += 1
            continue
        prose, desc_args = parse_desc(desc_raw)
        slot = save_dc_slot(prose, desc_args)
        if slot is None:
            buckets["no_DC_slot_with_int_plus_stat_argument"] += 1
            continue
        slot_n, expr, const, stat = slot

        if " ~ " not in key:
            buckets["orphan_key_is_not_monster_namespaced"] += 1
            continue
        owner_key = key.split(" ~ ", 1)[0]
        book_dir = os.path.dirname(path)
        owners = find_owner_row(book_dir, owner_key, cache)
        if not owners:
            buckets["orphan_no_owner_monster_row_in_this_book"] += 1
            continue
        if len(owners) > 1:
            buckets["owner_key_not_unique_in_book"] += 1
            continue
        owner_path, owner_line, owner_fields = owners[0]
        mc = token(owner_fields, "MONSTERCLASS")
        hd = racial_hd(mc)
        if hd is None:
            buckets["owner_racial_hd_unreadable"] += 1
            continue

        rule_base = 10 + hd // 2
        if rule_base != const:
            buckets["two_derivations_disagree"] += 1
            disagreements.append({
                "unit_id": unit["id"],
                "desc_argument": expr,
                "stated_save_dc_base": const,
                "owner_monster_key": owner_key,
                "owner_monster_class_token": mc,
                "universal_monster_rule_base": rule_base,
            })
            continue

        buckets["EMITTED"] += 1
        entries.append({
            "unit_id": unit["id"],
            "book": unit["book"],
            "record_key": key,
            "upstream_lst": os.path.relpath(path, root),
            "upstream_lst_sha256": sha256_file(path),
            "upstream_line": line_no,
            "corpus_field": "DESC:" + desc_raw,
            "desc_argument_index": slot_n,
            "desc_argument": expr,
            "owner_monster_key": owner_key,
            "owner_upstream_lst": os.path.relpath(owner_path, root),
            "owner_upstream_line": owner_line,
            "owner_monster_class_token": mc,
            "owner_racial_hd": hd,
            "universal_monster_rule_base": rule_base,
            "expected": {
                "save_dc_base": const,
                "ability": stat,
            },
        })

    print("monster_ability derived+grounded units considered:", len(units))
    for name, n in buckets.most_common():
        print("  %-46s %d" % (name, n))

    if disagreements:
        print()
        print("TWO DERIVATIONS DISAGREE -- not fixtured, not credited:")
        for d in disagreements:
            print("  %-58s DESC arg %-8s base %-3d | %s HD %s -> rule %d"
                  % (d["unit_id"], d["desc_argument"], d["stated_save_dc_base"],
                     d["owner_monster_key"], d["owner_monster_class_token"],
                     d["universal_monster_rule_base"]))

    if args.report:
        return

    with open(FIXTURE, encoding="utf-8") as fh:
        doc = json.load(fh)
    doc["monster_ability_token_family"] = "DESC (save-DC argument) x MONSTERCLASS (racial HD)"
    doc["monster_ability_derivation"] = (
        "PF1's Universal Monster Rules: 'The save DC against a monster's special ability is "
        "equal to 10 + 1/2 the monster's racial HD + the monster's relevant ability modifier.' "
        "PCGen states the already-summed `10 + 1/2 racial HD` term as the DESC token's `%N` "
        "argument for the placeholder the prose introduces with the word `DC`, spelled "
        "`<base>+<STAT>` or `<STAT>+<base>` (the ability-modifier term stays symbolic because it "
        "depends on the creature's live score). It states the racial HD itself on a DIFFERENT "
        "row, in a different file, as the trailing segment of `MONSTERCLASS:<type>:<HD>`. "
        "expected.save_dc_base is read off the ability row; universal_monster_rule_base is "
        "computed as 10 + racial_hd // 2 from the owning monster row; an entry is emitted ONLY "
        "when the two agree. Where they disagree neither side is known to be right, so the unit "
        "is not fixtured and not credited -- `--report` lists every such row."
    )
    doc["monster_ability_independence"] = (
        "Every monster_ability_entries value is a function of the upstream PCGen .lst bytes "
        "alone (upstream_lst/upstream_lst_sha256/upstream_line for the ability row, "
        "owner_upstream_lst/owner_upstream_line for the monster row). The generator imports no "
        "engine module, runs no engine binary, and opens no file under data/corpus/ -- which is "
        "the ingest the engine's monster_ability_save_dc() actually evaluates, through the "
        "compiled monster_chassis::MONSTER_BOOKS tables. The owner is resolved from PCGen's own "
        "`<Monster> ~ <Ability>` key namespacing against the ability file's OWN book directory, "
        "never from the engine's `owners` field: an ability with no monster row of its own book "
        "is an orphan and is excluded."
    )
    doc["monster_ability_entries"] = entries
    with open(FIXTURE, "w", encoding="utf-8") as fh:
        json.dump(doc, fh, indent=2, sort_keys=False)
        fh.write("\n")
    print()
    print("wrote %d monster_ability_entries to %s" % (len(entries), os.path.relpath(FIXTURE, REPO)))


if __name__ == "__main__":
    main()
