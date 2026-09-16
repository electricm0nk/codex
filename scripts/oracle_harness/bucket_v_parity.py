#!/usr/bin/env python3
"""SD-35 AT-35-E4-002 -- the bucket-V oracle run (`epic-breakdown.md § AT-35-E4-002`).

Bucket V is the 392 units the atlas classified `literal-verified` (389) / `fixture-verified`
(3) at `38b67db94e` -- the units whose magnitude a probe *observed* but which no oracle had
ever confirmed. This module runs them through the oracle **once**, comparing the values the
**live evaluator** renders (`src/bin/sheet_rule_bucket_v_render.rs`, `render_sheet` over
`data/sheet_rules/` -- not the old string-formula path) against PCGen at the pin.

Two oracle tiers, booked separately and never conflated -- `AGENTS.md` rule 7 ("a proof is
only as wide as the cases it covers"):

  `source`   the record's own row in the pinned PCGen checkout, located by the unit's
             `source_file` + `source_line` (both carried by `docs/work-inventory.json`, both
             re-checked here: the row's `KEY:`/name must match the unit's `corpus_key`). This
             tier reaches **every** unit and answers "does the number the sheet prints appear
             in the number PCGen's own data declares for this record". It is a data oracle: it
             does not exercise PCGen's engine.
  `export`   PCGen's BatchExporter, run over a carrier character that actually holds the unit
             (`sheet_parity.py`'s `charbuild_remainder_run_one.sh` path, the pinned install).
             This tier exercises the engine and is the stronger proof; it reaches only the
             units a carrier character can be built for.

Verdicts, per unit (`epic-breakdown.md`'s vocabulary):

  `oracle-agree`         every number the live evaluator printed for the unit is a number the
                         oracle declares for the same record.
  `oracle-disagree`      the oracle declares a number in that role and ours is not among them.
  `oracle-unverifiable`  the oracle prints no number in that role (the reason is named), or
                         the sheet line carries no magnitude at all (`words` with no number --
                         under the sheet rule that is a finished line, not a gap).

Subcommands (run in order):

    python3 scripts/oracle_harness/bucket_v_parity.py units --out <dir>
        Re-derives the 392-unit set from the atlas partition applied to
        `git show <sha>:docs/work-inventory.json` (default `38b67db94e`, the head
        `decisions.md §16`'s hand-off names) and writes `bucket-v-units.json` + `ids.json`.

    cargo run --locked --release --bin sheet_rule_bucket_v_render -- \\
        --units <dir>/ids.json --output <dir>/ours.json

    python3 scripts/oracle_harness/bucket_v_parity.py carriers --units <dir> --out <dir>/carriers
        Builds one `.pcg` per campaign closure, each carrying every unit of that closure PCGen
        can be handed as an `ABILITY:` line (the `charbuild_remainder_generate.py` shape --
        many units per JVM start).

    python3 scripts/oracle_harness/bucket_v_parity.py export --carriers <dir>/carriers \\
        --out <dir>/exports [--jobs 3] [--limit N]

    python3 scripts/oracle_harness/bucket_v_parity.py compare --units <dir> --ours <dir>/ours.json \\
        --exports <dir>/exports --output <dir>/bucket-v-parity.json

This is a tool-side script: it reads PCGen's data and PCGen's export. Nothing here is imported
by live code.
"""

from __future__ import annotations

import argparse
import collections
import json
import os
import re
import subprocess
import sys
import time
from concurrent.futures import ThreadPoolExecutor

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(os.path.dirname(HERE))
PIN_FILE = os.path.join(os.path.dirname(HERE), "pcgen-oracle-pin.env")
RUN_ONE = os.path.join(HERE, "charbuild_remainder_run_one.sh")
FTL = os.path.join(HERE, "bucket-v-abilities.txt.ftl")

#: The commit whose `docs/work-inventory.json` carries bucket V at its full 392
#: (`decisions.md §16`'s hand-off head; every later inventory has V at 0 because Epic 3 closed
#: the bucket). The set is a historical population, so it is re-derived from a pinned tree.
DEFAULT_INVENTORY_SHA = "38b67db94e"

sys.path.insert(0, os.path.join(REPO, "scripts"))
import completion_atlas  # noqa: E402  (the partition of record; never a second status list)


# ---------------------------------------------------------------------------
# The unit set
# ---------------------------------------------------------------------------


def pcgen_repo_dir():
    return os.environ.get("PCGEN_REPO_DIR") or os.path.expanduser("~/workspace/repos/pcgen")


def oracle_sha():
    with open(PIN_FILE, encoding="utf-8") as f:
        for line in f:
            if line.startswith("PCGEN_ORACLE_SHA="):
                return line.split("=", 1)[1].split("#")[0].strip()
    raise SystemExit(f"no PCGEN_ORACLE_SHA in {PIN_FILE}")


def inventory_at(sha):
    text = subprocess.run(
        ["git", "show", f"{sha}:docs/work-inventory.json"], cwd=REPO, capture_output=True, text=True, check=True
    ).stdout
    return json.loads(text)


def bucket_v_units(sha=DEFAULT_INVENTORY_SHA):
    inv = inventory_at(sha)
    return [u for u in inv["units"] if completion_atlas._bucket_of(u) == "V"]


def cmd_units(args):
    units = bucket_v_units(args.sha)
    os.makedirs(args.out, exist_ok=True)
    lst = lst_index()
    enriched = []
    for u in units:
        row = pinned_row(u, lst)
        enriched.append({**u, "pinned": row})
    with open(os.path.join(args.out, "bucket-v-units.json"), "w", encoding="utf-8") as f:
        json.dump(
            {
                "generated_by": "bucket_v_parity.py units",
                "inventory_sha": args.sha,
                "PCGEN_ORACLE_SHA": oracle_sha(),
                "units": enriched,
            },
            f,
            indent=1,
            sort_keys=True,
        )
    with open(os.path.join(args.out, "ids.json"), "w", encoding="utf-8") as f:
        json.dump([u["id"] for u in units], f, indent=1)
    kinds = collections.Counter(u["kind"] for u in units)
    located = sum(1 for u in enriched if u["pinned"].get("row"))
    print(f"bucket_v_parity units: n={len(units)} inventory_sha={args.sha} located_in_pinned_data={located}")
    print(f"  by kind: {dict(sorted(kinds.items()))}")
    return 0


# ---------------------------------------------------------------------------
# The `source` oracle tier: the record's own row in the pinned checkout
# ---------------------------------------------------------------------------

_LST_INDEX = None


def lst_index():
    """`basename -> path` for every `.lst` in the pinned checkout. Fails closed on an ambiguous
    basename: a unit's `source_file` must name exactly one file at the pin."""
    global _LST_INDEX
    if _LST_INDEX is not None:
        return _LST_INDEX
    root = os.path.join(pcgen_repo_dir(), "data")
    hits = collections.defaultdict(list)
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames.sort()
        for name in sorted(filenames):
            if name.endswith(".lst"):
                hits[name].append(os.path.join(dirpath, name))
    _LST_INDEX = {k: v for k, v in hits.items()}
    return _LST_INDEX


_ROW_CACHE = {}


def _rows(path):
    if path not in _ROW_CACHE:
        with open(path, encoding="utf-8", errors="replace") as f:
            _ROW_CACHE[path] = f.read().splitlines()
    return _ROW_CACHE[path]


#: Tokens whose body is a magnitude PCGen's own data declares for the record. A number inside
#: one of these is a number the oracle asserts; a number inside `SOURCEPAGE`, or a `%1` slot
#: marker in a `DESC`, is not.
MAGNITUDE_TOKENS = (
    "BONUS:", "DEFINE:", "DR:", "SR:", "SPELLS:", "TEMPBONUS:", "CAST:", "COST:", "WT:",
    "DAMAGE:", "CRITMULT:", "CRITRANGE:", "RANGE:", "REACH:", "AC:", "ACCHECK:", "MAXDEX:",
    "SPELLFAILURE:", "HANDS:", "LEVEL:", "HD:", "MOVE:", "CR:", "VISION:", "ARMORTYPE:",
    "PLUS:", "USES:", "TIMES:", "DURATION:", "CASTTIME:", "SIZE:", "BASEQTY:", "CONTAINS:",
    # A spell record declares its magnitudes in prose-bearing tokens too: the sheet's spell
    # block prints "One object weighing 10 lbs. or less" from `TARGETAREA:` and the casting
    # time from `CASTTIME:`. Omitting them booked a real agreement as a disagreement.
    "TARGETAREA:", "SAVEINFO:", "COMPS:", "ASPECT:",
)

_NUM_RE = re.compile(r"(?<![\w.])[+-]?\d+(?:\.\d+)?(?![\w])")
#: A dice expression in final form ("1d6", "2d10") -- under the sheet rule that IS the printed
#: value, so it is compared as its own token rather than dissolved into two integers.
_DICE_RE = re.compile(r"(?<![\w])(\d+)d(\d+)(?![\w])", re.IGNORECASE)


def numbers_in(text):
    """Every comparable value in `text`: scalars as ints/floats, dice expressions as their own
    `"<n>d<m>"` token. A digit inside a dice expression is NOT also emitted as a scalar -- the
    scalar regex's word-boundary guards exclude it -- so "1d4+2" compares as {"1d4", 2}."""
    out = set()
    text = text or ""
    for m in _NUM_RE.finditer(text):
        s = m.group(0)
        out.add(int(float(s)) if float(s) == int(float(s)) else float(s))
    for m in _DICE_RE.finditer(text):
        out.add(f"{int(m.group(1))}d{int(m.group(2))}")
    return out


def pinned_row(unit, lst):
    """The unit's own row at the pin, plus the numbers its magnitude-bearing tokens declare.

    Fails closed and says why: `not-in-pinned-data`, `ambiguous-source-file`,
    `line-out-of-range`, `row-name-mismatch`."""
    src = unit.get("source_file")
    line_no = unit.get("source_line")
    paths = lst.get(src or "", [])
    if not paths:
        return {"reason": "not-in-pinned-data", "source_file": src}
    if len(paths) > 1:
        return {"reason": "ambiguous-source-file", "source_file": src, "paths": len(paths)}
    path = paths[0]
    rows = _rows(path)
    if not isinstance(line_no, int) or not (1 <= line_no <= len(rows)):
        return {"reason": "line-out-of-range", "source_file": src, "source_line": line_no, "rows": len(rows)}
    row = rows[line_no - 1]
    fields = [f for f in row.split("\t") if f.strip()]
    if not fields:
        return {"reason": "blank-row", "source_file": src, "source_line": line_no}
    name = fields[0].strip()
    key = None
    for f in fields:
        if f.startswith("KEY:"):
            key = f[len("KEY:") :].strip()
    corpus_key = (unit.get("corpus_key") or "").strip()
    identity = key or name
    matched = bool(corpus_key) and (identity == corpus_key or name == corpus_key or (key or "") == corpus_key)
    if not matched and corpus_key:
        # PCGen's own row syntaxes for a row that is not a plain declaration: a `.MOD` row
        # modifies an existing object (optionally prefixed `CATEGORY=<cat>|`), and a
        # `.COPY=<KEY>` row declares a copy whose KEY is the suffix. Both are the record's own
        # row -- the ingest points at them deliberately -- so the identity is read out of the
        # syntax rather than treated as a mismatch.
        stem = name.split("|", 1)[1] if name.startswith("CATEGORY=") and "|" in name else name
        if ".COPY=" in stem:
            copied = stem.split(".COPY=", 1)[1].strip()
            matched = copied == corpus_key
            key = key or copied
        elif stem.endswith(".MOD"):
            matched = stem[: -len(".MOD")].strip() == corpus_key
            key = key or stem[: -len(".MOD")].strip()
    tokens = {}
    numbers = set()
    for f in fields[1:]:
        head = f.split(":", 1)[0] + ":" if ":" in f else f
        if any(f.startswith(t) for t in MAGNITUDE_TOKENS):
            tokens.setdefault(head, []).append(f)
            numbers |= numbers_in(f.split(":", 1)[1] if ":" in f else f)
    desc = next((f[len("DESC:") :] for f in fields if f.startswith("DESC:")), None)
    return {
        "path": os.path.relpath(path, pcgen_repo_dir()),
        "source_line": line_no,
        "name": name,
        "key": key,
        "identity_matches_corpus_key": matched,
        "category": next((f[len("CATEGORY:") :].strip() for f in fields if f.startswith("CATEGORY:")), None),
        "type": next((f[len("TYPE:") :].strip() for f in fields if f.startswith("TYPE:")), None),
        "magnitude_tokens": tokens,
        "declared_numbers": sorted(numbers, key=str),
        "desc_numbers": sorted(numbers_in(desc), key=str) if desc else [],
        "has_desc": desc is not None,
        "row": row[:4000],
    }


# ---------------------------------------------------------------------------
# The `export` oracle tier: carrier characters through PCGen's BatchExporter
# ---------------------------------------------------------------------------

#: our book id -> the PCGen `CAMPAIGN:` closure that loads it, read from the pinned `.pcc`
#: chain by `campaign_closure()`. The `KEY:`/display divergence is handled by `campaign_key`.
sys.path.insert(0, HERE)
from campaign_key import campaign_line_value  # noqa: E402

#: our book id -> the `.pcc` that declares it at the pin, relative to `data/`. The closure the
#: `.pcg` needs is computed from that file's own `PRECAMPAIGN:` chain by `campaign_closure()`
#: -- never guessed, and never a superset (a wrong extra book changes what PCGen grants).
BOOK_PCC = {
    "core_rulebook": "pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc",
    "advanced_players_guide": "pathfinder/paizo/roleplaying_game/advanced_players_guide/advanced_players_guide.pcc",
    "advanced_class_guide": "pathfinder/paizo/roleplaying_game/advanced_class_guide/_advanced_class_guide.pcc",
    "advanced_race_guide": "pathfinder/paizo/roleplaying_game/advanced_race_guide/advanced_race_guide.pcc",
    "ultimate_combat": "pathfinder/paizo/roleplaying_game/ultimate_combat/_ultimate_combat.pcc",
    "ultimate_equipment": "pathfinder/paizo/roleplaying_game/ultimate_equipment/ultimate_equipment.pcc",
    "ultimate_magic": "pathfinder/paizo/roleplaying_game/ultimate_magic/_ultimate_magic.pcc",
    "ultimate_intrigue": "pathfinder/paizo/roleplaying_game/ultimate_intrigue/_ultimate_intrigue.pcc",
    "ultimate_wilderness": "pathfinder/paizo/roleplaying_game/ultimate_wilderness/_ultimate_wilderness.pcc",
    "occult_adventures": "pathfinder/paizo/roleplaying_game/occult_adventures/_occult_adventures.pcc",
    "adventurers_guide": "pathfinder/paizo/roleplaying_game/adventurers_guide/_adventurers_guide.pcc",
    "horror_adventures": "pathfinder/paizo/roleplaying_game/horror_adventures/_horror_adventures.pcc",
    "mythic_adventures": "pathfinder/paizo/roleplaying_game/mythic_adventures/_mythic_adventures.pcc",
    "pathfinder_unchained": "pathfinder/paizo/roleplaying_game/pathfinder_unchained/_pathfinder_unchained.pcc",
    "monster_codex": "pathfinder/paizo/roleplaying_game/monster_codex/_monster_codex.pcc",
    "bestiary_2": "pathfinder/paizo/roleplaying_game/bestiary_2/_bestiary_2_for_players.pcc",
    "bestiary_3": "pathfinder/paizo/roleplaying_game/bestiary_3/_bestiary_3_for_players.pcc",
    "bestiary_4": "pathfinder/paizo/roleplaying_game/bestiary_4/_bestiary_4.pcc",
    "bestiary_5": "pathfinder/paizo/roleplaying_game/bestiary_5/_bestiary_5.pcc",
    "inner_sea_world_guide": "pathfinder/paizo/campaign_setting/inner_sea_world_guide/inner_sea_world_guide.pcc",
    "inner_sea_magic": "pathfinder/paizo/campaign_setting/inner_sea_magic/inner_sea_magic.pcc",
    "inner_sea_races": "pathfinder/paizo/campaign_setting/inner_sea_races/_inner_sea_races.pcc",
    "inner_sea_gods": "pathfinder/paizo/campaign_setting/inner_sea_gods/_inner_sea_gods.pcc",
    "inner_sea_intrigue": "pathfinder/paizo/campaign_setting/inner_sea_intrigue/_inner_sea_intrigue.pcc",
    "book_of_the_damned_volume_1": "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_1/book_of_the_damned_volume_1.pcc",
    "ultimate_psionics": "pathfinder/dreamscarred_press/ultimate_psionics/ultimate_psionics.pcc",
}

#: `INCLUDESBOOKTYPE=Core Rules` resolves to this campaign at the pin.
CORE_RULES_CAMPAIGN = "Core Rulebook"

_PCC_INDEX = None
_CLOSURE_CACHE = {}


def pcc_index():
    """`campaign KEY (and display name) -> .pcc path` for every campaign at the pin, so a
    `PRECAMPAIGN:` clause naming a book we did not map still resolves."""
    global _PCC_INDEX
    if _PCC_INDEX is not None:
        return _PCC_INDEX
    out = {}
    root = os.path.join(pcgen_repo_dir(), "data", "pathfinder")
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames.sort()
        for name in sorted(filenames):
            if not name.endswith(".pcc"):
                continue
            path = os.path.join(dirpath, name)
            display = key = None
            with open(path, encoding="utf-8", errors="replace") as f:
                for line in f:
                    if line.startswith("CAMPAIGN:"):
                        display = line[len("CAMPAIGN:") :].strip()
                    elif line.startswith("KEY:"):
                        key = line[len("KEY:") :].strip()
            for n in (key, display):
                if n and n not in out:
                    out[n] = path
    _PCC_INDEX = out
    return out


def _pcc_names(path):
    display = key = None
    requires = []
    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            line = line.rstrip("\n")
            if line.startswith("CAMPAIGN:"):
                display = line[len("CAMPAIGN:") :].strip()
            elif line.startswith("KEY:"):
                key = line[len("KEY:") :].strip()
            elif line.startswith("PRECAMPAIGN:"):
                body = line[len("PRECAMPAIGN:") :].split("\t")[0]
                parts = body.split(",")
                try:
                    need = int(parts[0])
                except ValueError:
                    continue
                # `PRECAMPAIGN:N,<clause>,<clause>...`: N of the clauses must be loaded. The
                # clauses are alternatives only when N is smaller than their number; taking the
                # first N is the minimal chain, the same rule `charbuild_remainder_generate.py`
                # applied by hand.
                clauses = parts[1:]
                for clause in clauses[:need]:
                    if clause.startswith("INCLUDESBOOKTYPE="):
                        requires.append(CORE_RULES_CAMPAIGN)
                    elif clause.startswith("INCLUDES="):
                        requires.append(clause[len("INCLUDES=") :].strip())
    return key or display, display, requires


def campaign_closure(book):
    """The `CAMPAIGN:` lines a `.pcg` needs to load `book`, dependencies first, computed from
    the pinned `.pcc` chain. `None` when the book has no `.pcc` mapping at the pin."""
    if book in _CLOSURE_CACHE:
        return _CLOSURE_CACHE[book]
    rel = BOOK_PCC.get(book)
    if not rel:
        _CLOSURE_CACHE[book] = None
        return None
    path = os.path.join(pcgen_repo_dir(), "data", rel)
    if not os.path.exists(path):
        _CLOSURE_CACHE[book] = None
        return None
    order = []
    seen = set()
    index = pcc_index()

    def walk(p, depth=0):
        if depth > 12:
            return None
        name, _display, requires = _pcc_names(p)
        if name in seen:
            return name
        seen.add(name)
        for req in requires:
            rp = index.get(req)
            if rp is None:
                continue
            walk(rp, depth + 1)
        if name not in order:
            order.append(name)
        return name

    walk(path)
    _CLOSURE_CACHE[book] = order
    return order

PCG_TEMPLATE = """PCGVERSION:2.0

# System Information
{campaigns}VERSION:6.09.08.RC1
GAMEMODE:Pathfinder_RPG
CHARACTERTYPE:PC
PURCHASEPOINTS:N
AUTOSPELLS:Y

# Character Bio
CHARACTERNAME:{name}
PLAYERNAME:sd35-at-35-e4-002

# Character Attributes
STAT:STR|SCORE:14
STAT:DEX|SCORE:14
STAT:CON|SCORE:14
STAT:INT|SCORE:14
STAT:WIS|SCORE:14
STAT:CHA|SCORE:14
ALIGN:LN
RACE:Human

# Character Class(es)
CLASS:Fighter|LEVEL:20|SKILLPOOL:0

# Character Experience
EXPERIENCE:0
EXPERIENCETABLE:Medium
{abilities}"""


def carrier_of(unit):
    """`(carrier name, ABILITY line)` for a unit PCGen can be handed directly, else `(None,
    reason)`. Only `class_feature` / `race_trait` records carry a `CATEGORY:` PCGen accepts on
    an `ABILITY:` line; equipment, equipment modifiers and spells need an `EQUIP`/`SPELL`
    carrier this run does not build (named, not hidden)."""
    pin = unit.get("pinned") or {}
    book = unit["book"]
    if not campaign_closure(book):
        return None, f"no-campaign-closure-for-{book}"
    if unit["kind"] not in ("class_feature", "race_trait"):
        return None, f"no-ability-carrier-for-kind-{unit['kind']}"
    category = pin.get("category")
    key = pin.get("key") or pin.get("name")
    if not category or not key:
        return None, "pinned-row-declares-no-CATEGORY"
    if "|" in key or "\t" in key:
        return None, "key-not-expressible-on-a-pcg-line"
    return book, f"ABILITY:{category}|TYPE:NORMAL|CATEGORY:{category}|KEY:{key}"


#: The template line `carriers` replaces with the carrier's own category list.
CATEGORY_MARKER = "<#assign categories = [] />"


def categories_of(ability_line):
    """The `CATEGORY:` of one generated `ABILITY:` line."""
    for part in ability_line.split("|"):
        if part.startswith("CATEGORY:"):
            return part[len("CATEGORY:") :]
    return None


def cmd_carriers(args):
    doc = json.load(open(os.path.join(args.units, "bucket-v-units.json"), encoding="utf-8"))
    os.makedirs(args.out, exist_ok=True)
    groups = collections.defaultdict(list)
    skipped = collections.Counter()
    for u in doc["units"]:
        book, line = carrier_of(u)
        if book is None:
            skipped[line] += 1
            continue
        groups[book].append((u["id"], line))
    # The engine-side twin of `PCG_TEMPLATE`: the SAME character, in the `key=value` character
    # input format, so `sheet_rule_bucket_v_render --character` renders at the level and the
    # ability scores the carrier's `.pcg` carries. The Human ability-bonus choice is left
    # unmade on both sides (the `.pcg` carries no `ABILITY:Ability Bonus` line either), so the
    # scores are a flat 14 for both.
    with open(os.path.join(args.out, "carrier_character.txt"), "w", encoding="utf-8") as f:
        f.write(
            "# SD-35 AT-35-E4-002 -- the engine twin of every bucket-V carrier .pcg\n"
            "# (bucket_v_parity.py carriers). Human Fighter 20, every ability score 14.\n"
            "case_id=bucket-v-carrier\n"
            "source_package_id=pf1.core_rulebook\n"
            "race_id=race:human\n"
            "class_level=class:fighter:20\n"
            + "".join(f"ability={t}:14\n" for t in
                      ("strength", "dexterity", "constitution", "intelligence", "wisdom", "charisma"))
        )
    with open(FTL, encoding="utf-8") as f:
        template = f.read()
    if CATEGORY_MARKER not in template:
        raise SystemExit(f"{FTL} has no {CATEGORY_MARKER!r} marker")
    manifest = {}
    for book, entries in sorted(groups.items()):
        closure = campaign_closure(book)
        campaigns = "".join(f"CAMPAIGN:{campaign_line_value(c)}\n" for c in closure)
        abilities = "\n".join(sorted({line for _, line in entries})) + "\n"
        name = f"bucket_v_{book}"
        with open(os.path.join(args.out, f"{name}.pcg"), "w", encoding="utf-8") as f:
            f.write(PCG_TEMPLATE.format(name=name, campaigns=campaigns, abilities=abilities))
        categories = sorted({c for _, c in ((i, cat) for i, cat in ((i, categories_of(line)) for i, line in entries)) if c})
        literal = "[" + ", ".join('"' + c.replace('"', '\\"') + '"' for c in categories) + "]"
        with open(os.path.join(args.out, f"{name}.ftl"), "w", encoding="utf-8") as f:
            f.write(template.replace(CATEGORY_MARKER, f"<#assign categories = {literal} />", 1))
        manifest[name] = {
            "book": book,
            "campaigns": closure,
            "categories": categories,
            "units": sorted(i for i, _ in entries),
        }
    with open(os.path.join(args.out, "manifest.json"), "w", encoding="utf-8") as f:
        json.dump({"carriers": manifest, "not_carried": dict(sorted(skipped.items()))}, f, indent=1)
    carried = sum(len(v["units"]) for v in manifest.values())
    print(f"bucket_v_parity carriers: carriers={len(manifest)} units_carried={carried} not_carried={sum(skipped.values())}")
    for reason, n in sorted(skipped.items(), key=lambda x: -x[1]):
        print(f"    {n:4d}  {reason}")
    return 0


def run_one(pcg, out_txt, settings, ftl):
    log = out_txt + ".log"
    with open(log, "w", encoding="utf-8") as lf:
        proc = subprocess.run(["bash", RUN_ONE, pcg, ftl, out_txt, settings], stdout=lf, stderr=subprocess.STDOUT)
    return proc.returncode


def cmd_export(args):
    args.carriers = os.path.abspath(args.carriers)
    args.out = os.path.abspath(args.out)
    os.makedirs(args.out, exist_ok=True)
    pcgs = sorted(p for p in os.listdir(args.carriers) if p.endswith(".pcg"))
    if args.limit:
        pcgs = pcgs[: args.limit]
    jobs = [
        (
            os.path.join(args.carriers, p),
            os.path.join(args.out, p[: -len(".pcg")] + ".txt"),
            os.path.join(args.out, "_settings", p[: -len(".pcg")]),
            os.path.join(args.carriers, p[: -len(".pcg")] + ".ftl"),
        )
        for p in pcgs
    ]
    started = time.time()
    failures = 0
    with ThreadPoolExecutor(max_workers=args.jobs) as pool:
        for (pcg, out_txt, _, _), rc in zip(jobs, pool.map(lambda j: run_one(*j), jobs)):
            ok = rc == 0 and os.path.exists(out_txt)
            print(f"  {'ok  ' if ok else 'FAIL'} {os.path.basename(pcg)} exit={rc}")
            failures += 0 if ok else 1
    wall = time.time() - started
    print(f"bucket_v_parity export: carriers={len(jobs)} failed={failures} wall={wall:.1f}s -> {args.out}")
    return 1 if failures else 0


def parse_export(text):
    """The `bucket-v-abilities.txt.ftl` export: `ABILITY|<key>|<desc>` records, one per line."""
    out = {}
    for raw in text.splitlines():
        if not raw.startswith("ABILITY|"):
            continue
        parts = raw.split("|", 2)
        if len(parts) < 3:
            continue
        out[parts[1].strip()] = parts[2]
    return out


def load_exports(path):
    out = {}
    if not path or not os.path.isdir(path):
        return out
    for name in sorted(os.listdir(path)):
        if not name.endswith(".txt"):
            continue
        with open(os.path.join(path, name), encoding="utf-8", errors="replace") as f:
            out[name[: -len(".txt")]] = parse_export(f.read())
    return out


# ---------------------------------------------------------------------------
# The comparison
# ---------------------------------------------------------------------------


def ours_numbers(entry):
    """What the live evaluator printed for the unit, in the two roles a sheet line has.

    Returns `(form, role, numbers, reason)`:

      role `value`  the line's own magnitude (`printed`) and every `also` magnitude on it --
                    the "DC 15" / "1d8+2" the sheet prints in the value column.
      role `prose`  the numbers inside the rendered words, when the line has no magnitude of
                    its own. Those numbers are substituted by the same evaluator (a `Slot`
                    piece resolved from the rule's `Var` table), so they are the evaluator's
                    output too -- and they are what PCGen's substituted `DESCRIPTION` prints.
      role `None`   the line renders words carrying no number at all. Under the sheet rule
                    that is a finished line, not a gap: there is nothing to compare.
    """
    line = entry.get("line")
    if not line:
        if entry.get("suppressed"):
            # `print: false` on the rule: a grouping/container record whose children carry the
            # sheet lines. Nothing is printed for it, so there is nothing to compare.
            return None, None, [], "rule-is-print-false-nothing-reaches-the-sheet"
        if not entry.get("in_package"):
            return None, None, [], "not-in-the-sheet-rule-package"
        return None, None, [], "no-line-rendered"
    form = line.get("form")
    value_nums = set(numbers_in(line.get("printed") or ""))
    for extra in line.get("also") or []:
        if isinstance(extra, dict):
            value_nums |= numbers_in(extra.get("printed") or "")
        elif isinstance(extra, str):
            value_nums |= numbers_in(extra)
    if value_nums:
        return form, "value", sorted(value_nums, key=str), None
    prose_nums = numbers_in(line.get("prose") or "")
    if prose_nums:
        return form, "prose", sorted(prose_nums, key=str), None
    return form, None, [], "line-carries-no-number"


def cause_of(ours, role, oracle_nums):
    """A mechanical classification of a disagreement, so the receipt names each one by
    mechanism rather than by prose.

      `value-role-number-the-oracle-never-prints-words-agree`
          our value column carries a number, our rendered words carry only numbers the oracle
          also prints. The words agree; the value column is the whole disagreement.
      `rendered-words-disagree`
          a number inside our rendered words is not one the oracle prints. The sheet's prose
          is wrong, not just its value column.
    """
    line = (ours or {}).get("line") or {}
    prose_nums = numbers_in(line.get("prose") or "")
    if role == "value" and prose_nums and all(n in oracle_nums for n in prose_nums):
        return "value-role-number-the-oracle-never-prints-words-agree"
    return "rendered-words-disagree"


def verdict_for(unit, ours, exports_by_carrier, carrier_manifest):
    pin = unit.get("pinned") or {}
    form, role, nums, why = ours_numbers(ours or {})
    out = {
        "id": unit["id"],
        "kind": unit["kind"],
        "book": unit["book"],
        "corpus_key": unit.get("corpus_key"),
        "form": form,
        "role": role,
        "ours": nums,
        "pinned_row": pin.get("path"),
        "pinned_line": pin.get("source_line"),
        "identity_matches_corpus_key": pin.get("identity_matches_corpus_key"),
    }
    if not pin.get("row"):
        out.update(verdict="oracle-unverifiable", tier="source", reason=pin.get("reason", "no-pinned-row"))
        return out
    if not pin.get("identity_matches_corpus_key"):
        out.update(verdict="oracle-unverifiable", tier="source", reason="pinned-row-identity-mismatch")
        return out

    # --- the `export` tier, when a carrier actually exported this unit's ability ---
    key = pin.get("key") or pin.get("name")
    for carrier, meta in carrier_manifest.items():
        if unit["id"] not in meta["units"]:
            continue
        exported = exports_by_carrier.get(carrier) or {}
        desc = exported.get(key)
        if desc is None:
            out["export_reason"] = "pcgen-did-not-grant-the-ability"
            break
        oracle_nums = sorted(numbers_in(desc), key=str)
        out["oracle_export"] = oracle_nums
        if why:
            out.update(verdict="oracle-unverifiable", tier="export", reason=why)
            return out
        if not oracle_nums:
            out.update(verdict="oracle-unverifiable", tier="export", reason="export-desc-has-no-number")
            return out
        missing = [n for n in nums if n not in oracle_nums]
        out.update(
            verdict="oracle-agree" if not missing else "oracle-disagree",
            tier="export",
            missing=missing,
            reason=None if not missing else "ours-not-among-the-numbers-pcgen-printed",
            cause=None if not missing else cause_of(ours, role, oracle_nums),
        )
        return out

    # --- the `source` tier ---
    if why:
        out.update(verdict="oracle-unverifiable", tier="source", reason=why)
        return out
    declared = set(pin.get("declared_numbers") or []) | set(pin.get("desc_numbers") or [])
    out["oracle_source"] = sorted(declared, key=str)
    if not declared:
        out.update(verdict="oracle-unverifiable", tier="source", reason="pinned-row-declares-no-number")
        return out
    missing = [n for n in nums if n not in declared]
    out.update(
        verdict="oracle-agree" if not missing else "oracle-disagree",
        tier="source",
        missing=missing,
        reason=None if not missing else "ours-not-among-the-numbers-the-pinned-row-declares",
        cause=None if not missing else cause_of(ours, role, declared),
    )
    return out


def cmd_compare(args):
    doc = json.load(open(os.path.join(args.units, "bucket-v-units.json"), encoding="utf-8"))
    ours_doc = json.load(open(args.ours, encoding="utf-8"))
    ours = {u["id"]: u for u in ours_doc["units"]}
    manifest_path = os.path.join(args.units, "carriers", "manifest.json")
    carrier_manifest = json.load(open(manifest_path, encoding="utf-8"))["carriers"] if os.path.exists(manifest_path) else {}
    exports = load_exports(args.exports)

    rows = [verdict_for(u, ours.get(u["id"]), exports, carrier_manifest) for u in doc["units"]]
    counts = collections.Counter(r["verdict"] for r in rows)
    tiers = collections.Counter(f"{r['tier']}:{r['verdict']}" for r in rows)
    reasons = collections.Counter(r["reason"] for r in rows if r.get("reason"))
    disagreements = [r for r in rows if r["verdict"] == "oracle-disagree"]
    out = {
        "generated_by": "bucket_v_parity.py compare",
        "PCGEN_ORACLE_SHA": oracle_sha(),
        "inventory_sha": doc["inventory_sha"],
        "population": len(rows),
        "counts": dict(sorted(counts.items())),
        "by_tier": dict(sorted(tiers.items())),
        "unverifiable_reasons": dict(sorted(reasons.items())),
        "disagreements": disagreements,
        "units": rows,
    }
    with open(args.output, "w", encoding="utf-8") as f:
        json.dump(out, f, indent=1, sort_keys=True)
    print(
        f"bucket_v_parity compare: compared={len(rows)} "
        f"oracle_agree={counts.get('oracle-agree', 0)} "
        f"oracle_disagreement={counts.get('oracle-disagree', 0)} of {len(rows)} "
        f"oracle_unverifiable={counts.get('oracle-unverifiable', 0)} "
        f"PCGEN_ORACLE_SHA={oracle_sha()}"
    )
    print(f"  by tier: {dict(sorted(tiers.items()))}")
    for reason, n in sorted(reasons.items(), key=lambda x: -x[1]):
        print(f"    {n:4d}  {reason}")
    for d in disagreements:
        print(f"  DISAGREE {d['id']} ours={d['ours']} oracle={d.get('oracle_export') or d.get('oracle_source')} missing={d.get('missing')}")
    return 0


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=True)

    p = sub.add_parser("units")
    p.add_argument("--out", required=True)
    p.add_argument("--sha", default=DEFAULT_INVENTORY_SHA)
    p.set_defaults(func=cmd_units)

    p = sub.add_parser("carriers")
    p.add_argument("--units", required=True)
    p.add_argument("--out", required=True)
    p.set_defaults(func=cmd_carriers)

    p = sub.add_parser("export")
    p.add_argument("--carriers", required=True)
    p.add_argument("--out", required=True)
    p.add_argument("--jobs", type=int, default=3)
    p.add_argument("--limit", type=int, default=0)
    p.set_defaults(func=cmd_export)

    p = sub.add_parser("compare")
    p.add_argument("--units", required=True)
    p.add_argument("--ours", required=True)
    p.add_argument("--exports", default=None)
    p.add_argument("--output", required=True)
    p.set_defaults(func=cmd_compare)

    args = ap.parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
