#!/usr/bin/env python3
"""Classify a book's `companion` corpus ROWS before a round commits to it.

`docs/release/SD-29-corpus-wide-catch-up-lanes/decisions.md §45.1` is the reason
this file exists at all: a lane that ranks books by `docs/work-inventory.json`'s
*evidence token* is reading a statement about what the engine has compiled when
the question is what the **corpus rows** are, and the race-trait lane got its
whole successor queue exactly backwards that way.  `§46.1` applied the same
discipline to `monster_ability` and found a structural ceiling.  This is that
step for `companion`.

# The question this answers

The `companion` kind is not one kind.  A `.lst` basename carrying a
companion/familiar marker holds rows of three structurally different shapes, and
only one of them is a chassis:

* **creature** rows (`*_races_companion.lst`, `*_races_familiar.lst`) — a
  companion or familiar creature, the thing a player's sheet names.  This is the
  chassis: it carries `SIZE:`, `MOVE:`, `MONSTERCLASS:`, ability references.
* **ability** rows (`*_abilities_companion.lst`, `*_abilities_familiar.lst`,
  `*_abilities_race_*companion*.lst`) — a special quality, special attack or
  level-advancement package that reaches a player **only underneath the creature
  that owns it**, exactly as `monster_ability` does underneath `monster`.
* **class** rows (`*_classes_companion.lst`) — the PCGen `Companion` /
  `Familiar` monster *classes* the `MONSTERCLASS:` token names.  These are hit
  dice progressions, not creatures and not abilities.

An ability row no creature row claims is a record that loads and is never shown:
the stub class `§44.2` was written about.  The `ORPHAN` column below is that
count, and it is a **ceiling on the lane**, not a preference.

# The ownership shapes, every one stated by the corpus

(Six of them now.  Shapes 5 and 6 were each found by a round that had already
committed to a book and read the rows the classifier was about to throw away —
`decisions.md §56.1` and `§59.1`.  Both moved the lane's ceiling UP.)

1. **row-named** — a creature row's `ABILITY:Special Ability|AUTOMATIC|<name>`
   token names the ability outright (by `KEY:` or by display name).  This is the
   same predicate `transcribe_monster_tables.parse_special_ability_refs` uses.
2. **prerace** — the ability row's own `PRERACE:1,<Race>` names a creature row of
   this book.  `monster_ability` has no analogue; companions carry it heavily
   (every `TYPE:CompanionAdvancement` row does), and a classifier that only knew
   shape 1 would report them as orphans.
3. **prefix** — a namespaced `KEY:<Owner> ~ <Leaf>` whose `<Owner>` is a creature
   of this book, either verbatim or as the inner name of `Companion (<Owner>)` /
   `Familiar (<Owner>)`.  `Worg ~ Mastery` owns through `Companion (Worg)`.
4. **granted-by** — shape 1's *own* token, read on an **ability** row rather
   than on a creature row.  An `ABILITY:Special Ability|AUTOMATIC|<name>` token
   on an ability row that is itself owned grants `<name>` underneath the same
   creature, transitively.  Bestiary 1 is where this shape is unavoidable
   (`decisions.md §54.1`): `Companion Advancement ~ Dinosaur (Ankylosaurus)` is
   an ability row owned by `Companion (Dinosaur (Ankylosaurus))` through shape
   2, and it is the row — not the creature row — that names
   `Ankylosaurus ~ Stun`.  Applying shape 1 only to creature rows reports five
   of Bestiary 1's 35 ability rows as orphans while the corpus states their
   owner one hop away.
5. **display-name** (`decisions.md §56.1`) — shape 3's `<Owner>` read from the
   creature's `OUTPUTNAME:` rather than its `KEY:`.  `KEY:Kyton (Augur)`
   displays as `Augur` and its abilities are keyed `Augur ~ …`.  Read from the
   row's own token, never inferred by unwrapping the key's parentheses.
6. **relay** (`decisions.md §59.1`) — the owner is stated across a corpus row
   that is not an inventory unit.  Bestiary 4's `Familiar (Giant Flea)` names
   `Racial Traits ~ Flea (Giant)`, a `CATEGORY:Internal` row, and THAT row names
   `Flea (Giant) ~ Disease`.  Shape 4 walks unit-to-unit and cannot see it; see
   the block above `ability_refs_any_category` below.

A looser rule would over-report reachability, which is the direction that ships
stubs; each shape above is a token the row itself carries.  Shape 4 in
particular is **not** "an ability near another ability": it is shape 1's exact
token, and it only propagates from a row that already has an owner, so the
closure can never manufacture reachability out of an orphan pair.

Usage::

    python3 scripts/classify_companion_rows.py                 # every book
    python3 scripts/classify_companion_rows.py inner_sea_combat monster_codex

`PCGEN_CORPUS_ROOT` may point at a local PCGen ``data/`` checkout; it defaults to
``$HOME/workspace/repos/pcgen/data``.
"""

from __future__ import annotations

import json
import os
import re
import sys

INVENTORY = "docs/work-inventory.json"


def corpus_root() -> str:
    return os.environ.get(
        "PCGEN_CORPUS_ROOT", os.path.expanduser("~/workspace/repos/pcgen/data")
    )


def _rebase_under_pcgen_corpus_root(stale_absolute_path: str) -> str:
    """Strip whatever worktree-specific prefix a committed path carries up to
    (and including) its own `.../data/` segment, and re-root the remainder
    under this run's real `corpus_root()`.

    `docs/work-inventory.json`'s `corpus_root` and `additional_book_dirs`
    fields are both written as ABSOLUTE paths by whichever worktree last
    regenerated the inventory -- exactly the "oracle cited by literal local
    path" shape `AGENTS.md` forbids. Every fresh worktree's `PCGEN_CORPUS_ROOT`
    points at that same `pcgen/data` directory under a DIFFERENT worktree
    path, so the relative structure below `.../data/` is what actually
    transfers; the absolute prefix above it never does.
    """
    marker = f"{os.sep}data{os.sep}"
    idx = stale_absolute_path.find(marker)
    if idx == -1:
        # No `/data/` segment to rebase from (e.g. already relative, or a
        # differently-shaped path) -- pass it through unchanged rather than
        # guess.
        return stale_absolute_path
    relative_suffix = stale_absolute_path[idx + len(marker) :]
    return os.path.join(corpus_root(), relative_suffix)


def book_dirs() -> dict[str, str]:
    """Book id -> its PCGen source directory.

    **SD-32 T9-onboarding fix (`decisions.md §17a`-shaped correction):** this
    used to read `inv["corpus_root"]` and `inv["additional_book_dirs"]`
    literally -- both ABSOLUTE paths baked into `docs/work-inventory.json` by
    whichever worktree last regenerated it, exactly the "oracle cited by
    literal local path" shape `AGENTS.md` forbids. Every fresh worktree's
    `PCGEN_CORPUS_ROOT` points somewhere that stale path does not (the oracle
    slot is git-ignored and re-fetched per-worktree), so
    `transcribe_companion_tables.py` raised `FileNotFoundError`
    unconditionally on a fresh checkout. Both fields are now rebased under
    this run's own `corpus_root()` (already `PCGEN_CORPUS_ROOT`-env-var-aware,
    and already how the sibling `classify_monster_ability_rows.py::book_dirs`
    resolves its root) via `_rebase_under_pcgen_corpus_root`.
    """
    inv = json.load(open(INVENTORY, encoding="utf-8"))
    dirs: dict[str, str] = {}
    root = _rebase_under_pcgen_corpus_root(inv["corpus_root"])
    for entry in os.listdir(root):
        if os.path.isdir(os.path.join(root, entry)):
            dirs[entry] = os.path.join(root, entry)
    for extra in inv.get("additional_book_dirs", []):
        rebased = _rebase_under_pcgen_corpus_root(extra)
        dirs[os.path.basename(rebased)] = rebased
    return dirs


def row_shape(source_file: str) -> str:
    """Which of the three structural shapes a companion `.lst` basename holds."""
    if "_races_" in source_file:
        return "creature"
    if "_classes_" in source_file:
        return "class"
    return "ability"


def read_row(path: str, line_no: int) -> list[str]:
    with open(path, encoding="utf-8", errors="replace") as handle:
        line = handle.read().split("\n")[line_no - 1]
    return [t.strip() for t in line.split("\t") if t.strip()]


# `docs/work-inventory.json` records `source_file` as a BASENAME, so a `.lst`
# that PCGen loads out of a subdirectory (`support/`, `_pfs/`) is not at
# `<book>/<basename>` at all.  Bestiary 5's `b5_races_companion_oa.lst` is the
# first companion instance: two of its 57 units live in `support/`.
#
# `classify` used to `continue` past a path it could not open, which is the
# failure shape `decisions.md §47.3` warns about — a check that silently
# measures less than it claims.  Both consumers now resolve the basename and
# raise when it is nowhere under the book.
_RESOLVED: dict[tuple[str, str], str] = {}

# `decisions.md §9`: `core_essentials` is not a book.  `SD31-ATTRIB-*` moves a
# unit's REPORTING `book` field to the book its own `SOURCELONG:` header names
# (`ce_races_familiar_cr.lst` says `SOURCELONG:Bestiary`, so its 53 rows report
# `bestiary`), but the unit's `source_file` names a physical file that never
# moves -- it stays under `core_essentials`'s own PCGen directory forever.
#
# Without the fallback below, `resolve_source_file` walks only the reporting
# book's own root and a re-attributed unit's file is unreachable under ANY
# book's root.  Confirmed live before this widening:
#
#   python3 scripts/transcribe_companion_tables.py bestiary
#   -> "ce_races_familiar_cr.lst is nowhere under .../roleplaying_game/bestiary"
#
# which is the identical failure `SD31-E6-F9-002` hit in the monster lane and
# fixed the identical way (`transcribe_monster_tables.py`'s `_CORE_ESSENTIALS_DIR`
# / `gen_book_cache.rs`'s `CORE_ESSENTIALS_RELATIVE`).  Term for term the same
# rule, so the two transcribers and the Rust generator can never disagree about
# which file a citation names.
_CORE_ESSENTIALS_DIR = "pathfinder/paizo/roleplaying_game/core_essentials"


def resolve_source_file(directory: str, source_file: str) -> str:
    """`<book>/<basename>` if it exists, else the one match anywhere below it.

    Falls back to `core_essentials`'s own directory when the name is absent from
    `directory` entirely -- the one other place a `decisions.md §9`-re-attributed
    unit's real file can live.  The book's own root always wins (two separate
    passes, never one merged walk), so a book that legitimately owns a
    same-named file is never redirected to `core_essentials`' copy, and
    `core_essentials` never falls back to itself.
    """
    cached = _RESOLVED.get((directory, source_file))
    if cached is not None:
        return cached
    direct = os.path.join(directory, source_file)
    if os.path.exists(direct):
        _RESOLVED[(directory, source_file)] = direct
        return direct
    hits = [
        os.path.join(parent, source_file)
        for parent, _dirs, files in os.walk(directory)
        if source_file in files
    ]
    if not hits:
        ce_root = os.path.join(corpus_root(), _CORE_ESSENTIALS_DIR)
        if os.path.abspath(ce_root) != os.path.abspath(directory):
            hits = [
                os.path.join(parent, source_file)
                for parent, _dirs, files in os.walk(ce_root)
                if source_file in files
            ]
    if not hits:
        raise SystemExit(f"{source_file} is nowhere under {directory}")
    if len(hits) > 1:
        raise SystemExit(f"{source_file} is ambiguous under {directory}: {hits!r}")
    _RESOLVED[(directory, source_file)] = hits[0]
    return hits[0]


# A `.lst` whose pcc load line carries `PRECAMPAIGN:` is loaded only when that
# campaign is also loaded — and the gate is on the PCC LINE, never inside the
# `.lst` (`loop-instruction.md`, "Conditional cross-book support files"; a
# `grep PRECAMPAIGN` over the `.lst` itself returns 0).
#
# Most such gates name a book this repo HAS ingested (`INCLUDES=Bestiary 3`,
# `INCLUDES=Advanced Player's Guide`), so "gated" alone is not "out of scope".
# What is out of scope is a gate naming a campaign this repo has not ingested.
# `decisions.md §47.2` already ruled exactly this for Horror Adventures'
# `ha_abilities_race_oa.lst`, and `RuleSetId::Ha`'s doc comment records it.
#
# Row-19 desktop reach/catalog reds (SD-32, 2026-08-24): Occult Adventures
# left this tuple 2026-08-24 -- it IS an ingested book now
# (`reach_gate.rs::CORPUS_BOOK_IDS` carries `("occult_adventures",
# "occult_adventures")`, landed by an SD-31 wave-4 lane; `RuleSetId::Oa`
# drives real `feats`/`spells`/`class_feature` reach claims). Its own
# module doc comments (e.g. `bestiary_5/companion_data.rs`) that cited "not
# ingested" as the reason for a PRECAMPAIGN exclusion were stale the moment
# that landed; `decisions.md §27b` ("EVERYTHING", 2026-08-23) separately
# overturned "not applicable to the modelled campaign set" as an INGEST
# disposition for this exact shape (occult_adventures-gated monster_ability
# units) -- the same reasoning applies here. Leave this tuple EMPTY, not
# repopulated with a different name, unless a future book is genuinely
# unmodelled by this repo's ingest roster; check `CORPUS_BOOK_IDS` before
# adding one back.
UNINGESTED_CAMPAIGN_GATES: tuple[str, ...] = ()

_PCC_LOAD = re.compile(r"^[A-Z]+:(?P<path>[^|\t]+\.lst)\|(?P<rest>.*)$")


def precampaign_gates(directory: str) -> dict[str, str]:
    """`.lst` basename -> the `PRECAMPAIGN:` expression its pcc load line carries."""
    gates: dict[str, str] = {}
    for parent, _dirs, files in os.walk(directory):
        for name in files:
            if not name.endswith(".pcc"):
                continue
            with open(os.path.join(parent, name), encoding="utf-8", errors="replace") as handle:
                for line in handle:
                    line = line.strip()
                    if line.startswith("#"):
                        continue
                    match = _PCC_LOAD.match(line)
                    if match is None or "PRECAMPAIGN:" not in match.group("rest"):
                        continue
                    for field in match.group("rest").split("|"):
                        if field.startswith("PRECAMPAIGN:"):
                            gates[os.path.basename(match.group("path"))] = field
                            break
    return gates


def gated_on_an_uningested_campaign(gate: str | None) -> bool:
    return gate is not None and any(name in gate for name in UNINGESTED_CAMPAIGN_GATES)


def token(row: list[str], prefix: str) -> str | None:
    for field in row:
        if field.startswith(prefix):
            return field[len(prefix) :]
    return None


def special_ability_refs(row: list[str]) -> list[str]:
    """Keys named by `ABILITY:Special Ability|AUTOMATIC|…` (shape 1)."""
    refs: list[str] = []
    for field in row:
        if not field.startswith("ABILITY:Special Ability|"):
            continue
        for name in field.split("|")[2:]:
            name = name.strip()
            if not name or name.startswith("PRE") or "=" in name:
                continue
            if name not in refs:
                refs.append(name)
    return refs


def prerace_owners(row: list[str]) -> list[str]:
    """Creature names named by the row's own `PRERACE:` token (shape 2)."""
    owners: list[str] = []
    for field in row:
        if not field.startswith("PRERACE:"):
            continue
        parts = field[len("PRERACE:") :].split(",")
        for name in parts[1:]:
            name = name.strip().lstrip("!")
            if name and name not in owners:
                owners.append(name)
    return owners


def bare_species(key: str) -> str:
    """`Companion (Worg)` -> `Worg`; `Familiar (Seru)` -> `Seru`; else itself."""
    for wrapper in ("Companion (", "Familiar ("):
        if key.startswith(wrapper) and key.endswith(")"):
            return key[len(wrapper) : -1]
    return key


def species_index(creatures: list[dict]) -> dict[str, list[str]]:
    """`<bare species>` -> EVERY creature row of this book claiming that name.

    **A list, in creature row order, and both facts are load-bearing**
    (`decisions.md §59.3`).

    Until Bestiary 4 this was `{bare_species(k): k for k in creature_keys}` — a
    dict comprehension over a **set**, mapping each species to ONE key. Bestiary 4
    is the first book where a species name is claimed by TWO creature rows: it
    ships `Almiraj` **and** `Familiar (Almiraj)` (and the same pair for
    `Beheaded`, `Isitoq`, `Nycar`, `Pipefox`, `Pooka`, `Ratling`), and
    `bare_species` maps both to `Almiraj`.

    Two defects at once, and the first hid the second:

    * **Non-determinism.** Which of the two won was decided by set iteration
      order, i.e. by Python's per-process randomized string hash. The generated
      table differed run to run with no corpus change — caught by regenerating
      twice and diffing, which is the check the whole lane's
      "regenerate, don't hand-edit" proof rests on.
    * **A lossy answer even when it was stable.** `Nycar ~ Poison` is reached
      from `Nycar` AND from `Familiar (Nycar)`; the corpus states both rows and
      the catalog serves the ability under whichever creature the player is
      looking at. Attributing it to one was never a decision, just an artifact of
      the comprehension.

    Returning every claimant, in row order, is deterministic and states what the
    corpus states.
    """
    index: dict[str, list[str]] = {}
    for unit in creatures:
        index.setdefault(bare_species(unit["corpus_key"]), []).append(unit["corpus_key"])
    return index


def resolve_owner(
    name: str, display: dict[str, str], species: dict[str, list[str]]
) -> list[str]:
    """The creature keys `<name>` names: display name first, then every species claimant."""
    if name in display:
        return [display[name]]
    return species.get(name, [name])


# ---------------------------------------------------------------------------
# Shape 6, RELAY ROWS (`decisions.md §59.1`).
#
# Bestiary 4 is where the closure above stops one hop short.  Its creature row
# `Familiar (Giant Flea)` does not name `Flea (Giant) ~ Disease` at all; it
# names `Racial Traits ~ Flea (Giant)`, a `CATEGORY:Internal` row in the same
# `.lst`, and THAT row carries the
# `ABILITY:Special Ability|AUTOMATIC|Flea (Giant) ~ Disease|…` token.  The relay
# is a corpus row like any other, but it is **not an inventory unit** --
# `v06_work_inventory` does not count `CATEGORY:Internal` rows -- so shape 4,
# which walks unit-to-unit, has nothing to stand on, and the abilities read as
# orphans while the corpus states their owner two hops away.
#
# Two further things the corpus says here, both load-bearing:
#
# * The creature's own token is `ABILITY:Internal|AUTOMATIC|…`, not
#   `ABILITY:Special Ability|AUTOMATIC|…`.  PCGen's category segment names the
#   category of the keys that follow, so shape 1's `Special Ability`-only
#   predicate cannot see the first hop either.  Shape 6 reads the reference
#   under ANY category and resolves it ONLY against relay rows; shape 1 keeps
#   its narrower predicate for the unit-to-unit links it already governs.
# * `transcribe_companion_tables.parse_natural_attacks` has read this exact
#   token since round 1 and deliberately SKIPS entries containing ` ~ ` (they
#   are not attack names).  Those skipped entries are precisely the relays --
#   the token was already being read, one field at a time, for another purpose.
#
# This can never manufacture reachability: a relay is reached only from a
# creature row of this book, and a reached relay grants only what its own token
# names.  A relay no creature reaches grants nothing.
# ---------------------------------------------------------------------------


def ability_refs_any_category(row: list[str]) -> list[str]:
    """Keys named by `ABILITY:<Category>|AUTOMATIC|…` under ANY category.

    Shape 1's `special_ability_refs` is this token restricted to
    `Special Ability`; this is the same token read one category wider, used ONLY
    to resolve relay rows.
    """
    refs: list[str] = []
    for field in row:
        if not field.startswith("ABILITY:"):
            continue
        parts = field.split("|")
        if len(parts) < 3 or parts[1] != "AUTOMATIC":
            continue
        for name in parts[2:]:
            name = name.strip()
            if not name or name.startswith("PRE") or "=" in name:
                continue
            if name not in refs:
                refs.append(name)
    return refs


def _row_key(row: list[str]) -> str:
    """A corpus row's identity: its `KEY:` token, else its first field."""
    return token(row, "KEY:") or (row[0] if row else "")


def relay_rows(directory: str, abilities: list[dict]) -> dict[str, list[str]]:
    """Non-unit rows of this book's ability `.lst` files that name abilities.

    `<relay key> -> [<key it names>, ...]`, in file order.  A row whose key IS
    an inventory unit is excluded by construction: units are shapes 1-5's
    business and counting one here would double-count it.
    """
    unit_keys = {u["corpus_key"] for u in abilities}
    source_files: list[str] = []
    for unit in abilities:
        if unit["source_file"] not in source_files:
            source_files.append(unit["source_file"])
    relays: dict[str, list[str]] = {}
    for source_file in source_files:
        path = resolve_source_file(directory, source_file)
        with open(path, encoding="utf-8", errors="replace") as handle:
            for line in handle.read().split("\n"):
                if not line.strip() or line.lstrip().startswith("#"):
                    continue
                row = [t.strip() for t in line.split("\t") if t.strip()]
                key = _row_key(row)
                if not key or key in unit_keys or key in relays:
                    continue
                refs = ability_refs_any_category(row)
                if refs:
                    relays[key] = refs
    return relays


def relay_ownership(
    directory: str,
    creatures: list[dict],
    abilities: list[dict],
    creature_rows: dict[str, list[str]],
) -> dict[str, list[str]]:
    """`<ability unit key> -> [<creature key>, ...]` reached through relays."""
    relays = relay_rows(directory, abilities)
    if not relays:
        return {}
    ability_by_key = {u["corpus_key"]: u for u in abilities}
    ability_by_name = {u["name"]: u for u in abilities}

    # Seeded ONLY from creature rows of this book, in creature row order.
    reached: dict[str, list[str]] = {}
    for unit in creatures:
        for ref in ability_refs_any_category(creature_rows[unit["corpus_key"]]):
            if ref not in relays:
                continue
            owners = reached.setdefault(ref, [])
            if unit["corpus_key"] not in owners:
                owners.append(unit["corpus_key"])

    # A relay may name another relay.  Fixpoint in insertion order, so the
    # output can never depend on set iteration order.
    changed = True
    while changed:
        changed = False
        for key in list(reached):
            for ref in relays[key]:
                if ref not in relays:
                    continue
                owners = reached.setdefault(ref, [])
                for creature in reached[key]:
                    if creature not in owners:
                        owners.append(creature)
                        changed = True

    out: dict[str, list[str]] = {}
    for key, creature_owners in reached.items():
        for ref in relays[key]:
            hit = ability_by_key.get(ref) or ability_by_name.get(ref)
            if hit is None:
                continue
            got = out.setdefault(hit["corpus_key"], [])
            for creature in creature_owners:
                if creature not in got:
                    got.append(creature)
    return out


def classify(book: str, units: list[dict], directory: str) -> dict:
    gates = precampaign_gates(directory)
    gated = [
        u for u in units if gated_on_an_uningested_campaign(gates.get(u["source_file"]))
    ]
    gated_keys = {u["corpus_key"] for u in gated}
    units = [u for u in units if u["corpus_key"] not in gated_keys]

    creatures = [u for u in units if row_shape(u["source_file"]) == "creature"]
    abilities = [u for u in units if row_shape(u["source_file"]) == "ability"]
    classes = [u for u in units if row_shape(u["source_file"]) == "class"]

    creature_keys = {u["corpus_key"] for u in creatures}
    creature_species = species_index(creatures)

    # Shape 5, DISPLAY-NAME namespacing.  An ability row's `<X> ~ <Y>` prefix is
    # not always the creature's `KEY:`; it can be the creature's `OUTPUTNAME:`,
    # which is what a player actually sees.  Bestiary 3 is where this first
    # matters: `KEY:Kyton (Augur)` carries `OUTPUTNAME:Augur`, and its six
    # abilities are keyed `Augur ~ Spell-Like Abilities` and so on.  Six of that
    # book's creature rows are shaped this way and they own 19 ability rows
    # between them -- every single row the lane was about to write off as an
    # orphan (`decisions.md §56.1`).
    #
    # Read from the row's own `OUTPUTNAME:` token, never inferred by unwrapping
    # `Kyton (Augur)` into `Augur`: `bare_species` unwraps the two COMPANION
    # wrappers the corpus uses as wrappers (`Companion (`, `Familiar (`), while
    # `Kyton (Augur)` is a genus-and-species key whose parenthesis means
    # something else entirely.  Generalising the unwrap would have produced the
    # right answer here by luck and the wrong one for `Familiar (Fox)`.
    creature_display: dict[str, str] = {}
    for unit in creatures:
        path = resolve_source_file(directory, unit["source_file"])
        display = token(read_row(path, unit["source_line"]), "OUTPUTNAME:")
        if display:
            creature_display[display] = unit["corpus_key"]

    ability_by_key = {u["corpus_key"]: u for u in abilities}
    ability_by_name = {u["name"]: u for u in abilities}

    creature_rows: dict[str, list[str]] = {}
    for unit in creatures:
        creature_rows[unit["corpus_key"]] = read_row(
            resolve_source_file(directory, unit["source_file"]), unit["source_line"]
        )

    owned_row_named: set[str] = set()
    for unit in creatures:
        for ref in special_ability_refs(creature_rows[unit["corpus_key"]]):
            hit = ability_by_key.get(ref) or ability_by_name.get(ref)
            if hit is not None:
                owned_row_named.add(hit["corpus_key"])

    # Shape 6, relay rows — see the block above `ability_refs_any_category`.
    owned_relay = set(relay_ownership(directory, creatures, abilities, creature_rows))

    owned_prerace: set[str] = set()
    owned_prefix: set[str] = set()
    for unit in abilities:
        key = unit["corpus_key"]
        path = resolve_source_file(directory, unit["source_file"])
        row = read_row(path, unit["source_line"])
        for owner in prerace_owners(row):
            if owner in creature_keys or owner in creature_species or owner in creature_display:
                owned_prerace.add(key)
        if " ~ " in key:
            prefix = key.split(" ~ ")[0]
            if prefix in creature_keys or prefix in creature_species or prefix in creature_display:
                owned_prefix.add(key)

    # Shape 4, granted-by: shape 1's own token read on an ability row that is
    # itself already owned. Run to a fixpoint, because an advancement package
    # may name a row that names another; seeded ONLY from rows shapes 1-3
    # established, so an orphan can never grant reachability to another orphan.
    grants: dict[str, list[str]] = {}
    for unit in abilities:
        path = resolve_source_file(directory, unit["source_file"])
        row = read_row(path, unit["source_line"])
        named: list[str] = []
        for ref in special_ability_refs(row):
            hit = ability_by_key.get(ref) or ability_by_name.get(ref)
            if hit is not None and hit["corpus_key"] != unit["corpus_key"]:
                named.append(hit["corpus_key"])
        grants[unit["corpus_key"]] = named

    owned = owned_row_named | owned_prerace | owned_prefix | owned_relay
    owned_granted: set[str] = set()
    frontier = list(owned)
    while frontier:
        key = frontier.pop()
        for granted in grants.get(key, []):
            if granted not in owned:
                owned.add(granted)
                owned_granted.add(granted)
                frontier.append(granted)

    orphans = [u["corpus_key"] for u in abilities if u["corpus_key"] not in owned]
    # Delta rows (`decisions.md §59.2`).  A `<Base>.COPY=<Variant>` row states a
    # DELTA on another record, and a `.MOD` row an update to one; neither is a
    # record the chassis can transcribe from its own citation, so
    # `transcribe_companion_tables` drops both.  The inventory already classifies
    # them in its own `origin` field, so this reads a classification rather than
    # re-deriving one.  Counted over the whole unit set, creature rows included:
    # a `.COPY=` creature row is no more transcribable than a `.COPY=` ability
    # row, and Bestiary 2's monster half is where that first showed.
    deltas = [u["corpus_key"] for u in units if u.get("origin") in ("copy", "mod_only")]
    return {
        "book": book,
        "creatures": len(creatures),
        "abilities": len(abilities),
        "classes": len(classes),
        "row_named": len(owned_row_named),
        "prerace": len(owned_prerace),
        "prefix": len(owned_prefix),
        "relay": len(owned_relay),
        "granted": len(owned_granted),
        "orphans": orphans,
        "deltas": deltas,
        # The exclusions as a UNION, never a sum: a row can be both an orphan
        # and a `.COPY=` delta, and adding the columns would subtract it twice.
        # `§51.1` ruled that a ceiling subtracting one exclusion is not a
        # ceiling; `§54.2` moved the class-row subtraction out of prose and into
        # this instrument. This is the same correction one step further —
        # the arithmetic, not just the terms.
        "excluded": set(orphans)
        | set(deltas)
        | {u["corpus_key"] for u in classes}
        | {u["corpus_key"] for u in gated},
        "gated": [
            (u["corpus_key"], u["source_file"], gates[u["source_file"]]) for u in gated
        ],
    }


def main() -> None:
    inv = json.load(open(INVENTORY, encoding="utf-8"))
    dirs = book_dirs()
    wanted = sys.argv[1:]
    units_by_book: dict[str, list[dict]] = {}
    for unit in inv["units"]:
        if unit["kind"] == "companion":
            units_by_book.setdefault(unit["book"], []).append(unit)
    books = wanted or sorted(units_by_book)
    print(
        f"{'book':32} {'crea':>5} {'abil':>5} {'clas':>5} {'named':>6} "
        f"{'prerace':>8} {'prefix':>7} {'relay':>6} {'granted':>8} {'ORPHAN':>7}"
    )
    total_orphans = 0
    total_units = 0
    total_gated = 0
    total_classes = 0
    total_deltas = 0
    total_excluded = 0
    for book in books:
        units = units_by_book.get(book, [])
        if not units:
            print(f"{book:32} carries no companion units")
            continue
        directory = dirs.get(book)
        if directory is None:
            print(f"{book:32} no source directory found")
            continue
        result = classify(book, units, directory)
        total_orphans += len(result["orphans"])
        total_gated += len(result["gated"])
        total_classes += result["classes"]
        total_deltas += len(result["deltas"])
        total_excluded += len(result["excluded"])
        total_units += len(units)
        print(
            f"{book:32} {result['creatures']:5} {result['abilities']:5} "
            f"{result['classes']:5} {result['row_named']:6} {result['prerace']:8} "
            f"{result['prefix']:7} {result['relay']:6} {result['granted']:8} "
            f"{len(result['orphans']):7}"
        )
        if wanted and result["orphans"]:
            for key in result["orphans"]:
                print(f"    ORPHAN {key}")
        if wanted and result["gated"]:
            for key, source_file, gate in result["gated"]:
                print(f"    GATED  {key} — {source_file} loaded under {gate}")
    # `decisions.md §51.1`: "a ceiling that subtracts one exclusion is not a
    # ceiling; it is one exclusion." That section states this line already
    # subtracts all three known exclusions. It did not — the
    # `*_classes_companion.lst` class rows, which `transcribe_companion_tables`
    # refuses with a hard `SystemExit`, were still in the printed remainder, so
    # the number here read 886 while every doc downstream carried the hand-
    # corrected 879. `decisions.md §54.2` records the correction; the class-row
    # subtraction is now in the instrument rather than in prose.
    print(f"\ntotal companion units in scope : {total_units}")
    print(f"orphan ability rows            : {total_orphans}")
    print(f"PRECAMPAIGN-gated on an uningested campaign : {total_gated}")
    # "drops" rather than "refuses", corrected in round 8 (`decisions.md
    # §65.1`). Until that round the transcriber literally refused -- a
    # `SystemExit` -- on any book carrying a class row, and this label was
    # accurate. It now screens them, names them in the emitted module doc and
    # continues, which is why Core Rulebook could be ingested at all. The
    # excluded COUNT is unchanged in both cases; only the disposition moved.
    print(f"`*_classes_companion.lst` class rows the chassis drops : {total_classes}")
    print(f"`.COPY=`/`.MOD` delta rows the chassis drops : {total_deltas}")
    print(f"distinct excluded rows (the UNION, not the sum) : {total_excluded}")
    print(f"reachable remainder            : {total_units - total_excluded}")


if __name__ == "__main__":
    main()
