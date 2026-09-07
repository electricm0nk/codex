#!/usr/bin/env python3
"""Build the SD-32 Gate 1 shape ledger: classify every not-done unit into
one of the ten d20 compute-shape families SD-31 wave 31 measured (or an
honest vocabulary extension), and fail closed rather than silently report
false 100% coverage.

Why this exists
----------------
SD-31 wave 31 (`docs/release/SD-31-corpus-closure-grind/artifacts/MEASURE-TWICE.md`
§3) hand-measured the corpus's compute-shape families and found the ceiling
on what a compute library buys: 3,201 of 24,914 not-done units (12.8%),
because the single largest family -- 1,747 flat-literal-constant units --
gets zero benefit from any shared function. That measurement was run twice,
independently reproduced by an adversarial reviewer, and explicitly **not
committed as a script** ("measurement-only wave, no code/tooling changes
banked", MEASURE-TWICE.md §7). This script is that codification: the first
committed, re-runnable version of the classification, so Gate 1 is a
command with an exit code rather than a hand-derived document nobody can
re-check without redoing the walk by hand.

`decisions.md` Decision 4 is binding on the shape of this tool: the
**procedure** (extract value-bearing expressions / normalise / cluster /
count / report) is portable across systems; the **PF1e family vocabulary**
(the ten families below, "d20 families" per Decision 4) is not. This script
implements the procedure and ships the PF1e binding as data, the same way
`coverage_ledger.py` ships its group classification as a JSON table rather
than code -- except here the family classifier is a fixed, documented rule
list (not an external file) because AT-32-G1-003 requires every family to
carry a *stated proof width*, and a rule an operator cannot read in one
place is a proof width nobody can audit.

Extraction method (join, not re-derivation from scratch)
----------------------------------------------------------
`docs/work-inventory.json` units carry `book`, `source_file` (an LST
basename), and `source_line` (a 1-based line number in that LST file) but
no formula text of their own. The formula-bearing text lives in the
ingested corpus record under `data/corpus/<book>/<kind>/*.json`, whose
`source.path` (basename) / `source.line` identify the same LST row, and
whose `data.raw_tokens` list carries `{"key": ..., "value": ...}` pairs for
every `DEFINE:` / `BONUS:*` token PCGen's LST format attaches to that row
-- exactly the fields MEASURE-TWICE.md §7 named ("summing raw_tokens with
key=='DEFINE' or key.startswith('BONUS'), joined ... on (book, source_file,
source_line)"). This script builds that join once (`build_corpus_index`),
then classifies every DEFINE/BONUS value's formula segment
(`extract_formula_segment`) into a family (`classify_formula`).

A unit whose join finds no corpus record, or whose record carries no
DEFINE/BONUS token, is not "unclassified" -- it is classified into the
explicit **F0 (no formula content)** extension family. Most not-done units
carry no formula at all (24,914 not-done total vs. 4,948 formula-bearing
per MEASURE-TWICE.md §3's primary partition), so a ledger that only
classifies formula-bearing units and calls the rest "unclassified" would
violate AT-32-G1-001/002 on its very first run. F0 is named, counted, and
proof-width-stated the same as F1..F10 -- this is the "vocabulary extension
allowed with measured units" AT-32-G1-001 and `kanban.md` #5 explicitly
permit, not a loophole.

Family list, priority order, and proof width
-----------------------------------------------
`FAMILIES` below is the ordered rule list. A unit's *primary* family is the
first rule (in this order) whose predicate matches any of its DEFINE/BONUS
formula segments -- mirroring MEASURE-TWICE.md §3's own framing ("a unit's
PRIMARY family is assigned by priority order when it carries more than one
shape"). The order here is this script's own first codification (the
original wave-31 walk was hand-run and not committed, per this docstring's
opening paragraph) and is deliberately most-specific-keyword-first, so a
formula is never miscredited to a generic bucket when a specific one
applies (e.g. a `skillinfo(...)` expression that also contains a bare
`ClassLVL` term is skill-rank-derived, not per-level-scaling, because the
`skillinfo` grammar is what actually gates it).

Every family below states, in its `proof_width` field, which corpus shapes
its predicate covers and which it knowingly does not -- required by
AT-32-G1-003 after the SD-31 wave 21 lesson: a proof that agreed with its
own reference set on 100% of cases was still fabricating 73.4% of its
output because the reference set never exercised the shapes it got wrong.

Usage
-----
    python3 scripts/shape_ledger.py --inventory docs/work-inventory.json \\
        --output artifacts/gate-1-shape-closure/ledger.json

Exit status is 0 on a normal run (even with rows in every family) and 1 --
printing "no coverage: ..." to stderr -- when the inventory cannot be read
or parsed as JSON, or when it parses but its `units` population is empty.
This is the fail-closed posture AT-32-G1-002 requires, mirroring
`coverage_ledger.py`'s posture for the 46-group partition: an empty or
absent predicate must never be allowed to manufacture a false "0
unclassified" result by having nothing to classify.
"""

from __future__ import annotations

import argparse
import glob
import json
import os
import re
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DEFAULT_INVENTORY = os.path.join(REPO_ROOT, "docs", "work-inventory.json")
DEFAULT_CORPUS_ROOT = os.path.join(REPO_ROOT, "data", "corpus")

sys.path.insert(0, os.path.join(REPO_ROOT, "scripts", "observer"))
import pf1e_dashboard_producer as P  # noqa: E402  (path set above)

# Reuse coverage_ledger.py's not-done population definition (same
# EXCLUDED_BOOKS, same doneness_verdict) so Gate 1's population matches the
# figure every other gate/epic cites (epic-breakdown.md's 24,914).
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import coverage_ledger as CL  # noqa: E402

# `decisions.md §27a`: "F0 reached by measurement is a real answer; F0
# reached by 'nothing else matched' is a placeholder wearing a family
# label." The redaction marker is the ONE shared literal every PI-redaction
# path in this program writes (`scripts/pi_scrub.py::REDACTED_PI_MARKER`) --
# imported, never re-typed, per `AGENTS.md`/this bundle's dispatch brief.
import pi_scrub as PS  # noqa: E402

_ABILITY_ABBREVS = ("STR", "DEX", "CON", "INT", "WIS", "CHA")

# Stat/skill token names PCGen formulas use that are NOT per-level-scaling
# even though they can appear alongside a bare word -- excluded from the
# F2 per-level-scaling word-boundary match so "STR" alone never false-
# -positives as "<Word>LVL".
_LVL_RE = re.compile(r"\b[A-Za-z][A-Za-z0-9_]*LVL\b")
_CLASSLEVEL_RE = re.compile(r"\bclasslevel\s*\(", re.IGNORECASE)
_SKILLRANK_RE = re.compile(r"\bskillinfo\s*\(|\bTOTALRANK\b", re.IGNORECASE)
_MINMAX_RE = re.compile(r"\b(min|max|floor|ceil)\s*\(", re.IGNORECASE)
_IF_RE = re.compile(r"\bif\s*\(", re.IGNORECASE)
_THRESHOLD_STEP_RE = re.compile(r">=\s*\d+")
_FLAT_CONST_RE = re.compile(r"^-?\d+(\.\d+)?%?$")
_BARE_ABILITY_RE = re.compile(
    r"^-?(" + "|".join(_ABILITY_ABBREVS) + r")([+\-]\d+(\.\d+)?)?$"
)
_PLAIN_IDENTIFIER_RE = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*$")


def _is_level_threshold_step_count(formula: str) -> bool:
    """T10-style shape: two or more `if(...>=N,1,0)`-style terms summed
    together. Rare (7 units in MEASURE-TWICE.md §3) and the hardest to
    detect from a single formula string without a real parser -- this is a
    deliberately narrow heuristic (>=2 threshold comparisons AND >=2 `if(`
    calls AND a top-level `+` joining them), not a claim of full recall.
    """
    if formula.count("if(") < 2 and not re.search(r"\bif\s*\(.*\bif\s*\(", formula, re.IGNORECASE):
        return False
    thresholds = _THRESHOLD_STEP_RE.findall(formula)
    return len(thresholds) >= 2 and "+" in formula


# Ordered classification rules. Each entry: (family_id, label, predicate,
# proof_width). `predicate(formula)` receives one DEFINE/BONUS formula
# segment (already extracted, never the raw "KEY|..." token) and returns
# True/False. Order matters -- first match wins (MEASURE-TWICE.md §3's
# "primary family assigned by priority order").
FAMILIES = [
    (
        "F9",
        "Skill-rank-derived (skillinfo/TOTALRANK)",
        lambda f: bool(_SKILLRANK_RE.search(f)),
        "Covers any formula segment containing the `skillinfo(` call or a "
        "bare `TOTALRANK` reference. Does not cover skill-rank use inside "
        "a `.MOD`-continuation row this script's join never reaches (see "
        "module docstring 'Extraction method'), and does not distinguish "
        "class-skill vs. cross-class skillinfo variants.",
    ),
    (
        "F6",
        "classlevel(...)-derived",
        lambda f: bool(_CLASSLEVEL_RE.search(f)),
        "Covers any formula segment calling `classlevel(`. Known corpus "
        "defect (MEASURE-TWICE.md §3, `defects.md` D2): `classlevel()` "
        "does not verify its class-name argument, so a unit landing here "
        "is not automatically safe to bank through the interpreter until "
        "D2 is fixed -- this script only classifies the shape, it does "
        "not certify the value.",
    ),
    (
        "F10",
        "Level-threshold step-count (summed >= indicators)",
        _is_level_threshold_step_count,
        "Narrow heuristic: >=2 `if(` calls AND >=2 numeric `>=` "
        "comparisons AND a top-level `+` in the same formula segment. "
        "Does not cover a step-count expressed across multiple separate "
        "DEFINE/BONUS tokens summed by the consumer rather than within "
        "one formula string -- that shape is invisible to a per-token "
        "classifier and would need a per-record aggregation pass.",
    ),
    (
        "F5",
        "Clamped/capped per-level scaling (min/max/floor/ceil around a level expr)",
        lambda f: bool(_MINMAX_RE.search(f)) and bool(_LVL_RE.search(f)),
        "Covers `min(`/`max(`/`floor(`/`ceil(` wrapping a formula segment "
        "that also contains a `<Word>LVL` term. Does not cover a clamp "
        "wrapping a named-counter variable with no literal LVL token in "
        "the same segment (e.g. a clamp on an accumulator whose own LVL "
        "dependency lives in a separate DEFINE row) -- that case falls "
        "through to F4 or F8 below, undercounting F5 by construction.",
    ),
    (
        "F2",
        "Per-level scaling (<Class>LVL bare or arithmetic)",
        lambda f: bool(_LVL_RE.search(f)),
        "Covers any formula segment containing a `<Word>LVL` token, bare "
        "or inside arithmetic. Does not distinguish a modelled class's "
        "own LVL variable from an unmodelled class's (T2a/T12 overlap, "
        "`epic-breakdown.md` 'What the measurement actually established') "
        "-- that distinction needs the class roster, which this script "
        "does not consult.",
    ),
    (
        "F7",
        "Conditional-step (if/boolean toggle)",
        lambda f: bool(_IF_RE.search(f)),
        "Covers any formula segment containing an `if(` call not already "
        "claimed by F9/F6/F10 above. Does not distinguish a genuine "
        "boolean toggle from an `if(...)` used purely for a units-safe "
        "default (e.g. `if(hasfeat,X,0)` vs. a real branch) -- both read "
        "as the same shape to a regex-level classifier.",
    ),
    (
        "F3",
        "Ability-modifier-derived (STR/DEX/CON/INT/WIS/CHA)",
        lambda f: bool(_BARE_ABILITY_RE.match(f.strip()))
        or any(re.search(rf"\b{a}\b", f) for a in _ABILITY_ABBREVS),
        "Covers a formula segment that is a bare ability abbreviation "
        "(optionally signed/offset) or contains one as a token. Does not "
        "cover an ability score referenced only by its long PCGen "
        "variable form (e.g. a custom-named stat-derived variable one "
        "level removed) -- that shape falls through to F4/F8.",
    ),
    (
        "F4",
        "Named-counter/pool variable (plain identifier reference)",
        lambda f: bool(_PLAIN_IDENTIFIER_RE.match(f.strip()))
        and f.strip().upper() not in _ABILITY_ABBREVS,
        "Covers a formula segment that is exactly one bare identifier "
        "(no arithmetic, no literal) and is not an ability abbreviation "
        "-- the shape `bonus_stack_reader.rs`'s binding-layer pattern "
        "targets (`technical-design.md` Gate 2). Does not cover a counter "
        "referenced inside arithmetic (`Counter+2`) -- that reads as F8 "
        "residual here, which undercounts F4 relative to a full "
        "producer-chain resolution (the 77.2%/893-of-1,156 figure in "
        "`epic-breakdown.md` Epic 1 comes from resolving producer chains, "
        "a heavier analysis than this per-formula classifier performs).",
    ),
    (
        "F1",
        "Flat-constant magnitude (bare literal)",
        lambda f: bool(_FLAT_CONST_RE.match(f.strip())),
        "Covers a formula segment that is exactly a signed/unsigned "
        "integer or decimal literal, optionally with a trailing `%`. "
        "Does not cover a literal embedded in a larger arithmetic "
        "expression (e.g. `2+CON` is F3, not F1, by design -- the point "
        "of this family per `epic-breakdown.md` is that it 'gets zero "
        "benefit from any shared function', which only holds for bare "
        "literals).",
    ),
]

# The extension family for units with no DEFINE/BONUS token content at
# all -- the honest majority (epic-breakdown.md: 24,914 not-done units,
# only 4,948 carry any formula shape at all in the primary partition).
FAMILY_F0_NO_FORMULA = "F0"
_FAMILY_F0_LABEL = "No formula content (no DEFINE/BONUS token found for this unit)"
_FAMILY_F0_PROOF_WIDTH = (
    "Covers a unit whose (book, source_file, source_line) join finds either "
    "no corpus record, or a corpus record whose data.raw_tokens carries no "
    "DEFINE/BONUS-prefixed key. Does not distinguish 'this unit genuinely "
    "has no computable magnitude' (a race description, a pure-text feat) "
    "from 'the join failed to find the record it should have' -- the "
    "`join_status` field on each ledger row (`no_record` vs `no_formula_"
    "tokens`) is what a reviewer checks to tell those apart; F0's own "
    "count does not."
)

# The residual bucket: a formula segment exists but matches none of the
# named rules above. Named explicitly (not silently folded into F0) so a
# reviewer can see the classifier's own blind spot rather than have it
# vanish into the "no formula" bucket.
FAMILY_F8_OTHER = "F8"
_FAMILY_F8_LABEL = "Other named-variable expression (residual)"
_FAMILY_F8_PROOF_WIDTH = (
    "Covers any non-empty formula segment that matched none of F1/F2/F3/"
    "F4/F5/F6/F7/F9/F10 above -- e.g. multi-term arithmetic mixing several "
    "identifiers, or a PCGen builtin this classifier does not recognise "
    "(var(\"...\"), PREVARGTEQ-gated expressions, COUNT[...] terms). This "
    "is the classifier's own named blind spot per AT-32-G1-003 -- a "
    "non-zero F8 count is expected and does not indicate a defect by "
    "itself, but a cycle that finds F8 dominating a book's population "
    "should treat that as a signal the rule list needs another entry, "
    "not silence it."
)


def load_inventory_or_die(path: str) -> dict:
    """Loads the inventory JSON. Raises on any failure -- callers must
    treat that as the fail-closed "no coverage" case, never as an implicit
    zero-unit, zero-unclassified pass."""
    with open(path, "r", encoding="utf-8") as fh:
        return json.load(fh)


# `docs/work-inventory.json` names this book `"bestiary"` (no trailing
# `a`) for every unit, but `data/corpus/`'s directory for it carries the
# historical `"beastiary"` spelling -- see `scripts/transcribe_monster_
# tables.py`'s `CROSS_TABLE_MONSTER_RECORDS = {"bestiary": "beastiary"}`
# and `src/bin/gen_book_cache.rs`'s `corpus_book: "beastiary"` (both
# already document this as deliberate, historical, and not something to
# rename out from under). Without this alias, every `bestiary` unit's
# join walks a near-empty `data/corpus/bestiary/` and reports
# `no_record` even though its real record exists one directory over --
# confirmed for real: 260 `bestiary` `monster_ability` units alone
# (`decisions.md §20`'s ledger re-derive).
BOOK_CORPUS_DIR_ALIASES: dict[str, str] = {
    "bestiary": "beastiary",
}

_GENERIC_KIND_DIR_SUFFIX = "_generic"


def normalize_kind_dir(raw_kind: str) -> str:
    """Normalizes a corpus directory's own kind segment for join purposes.
    `scripts/ingest_generic_kind.py` (`race`/`monster`/`class`, and now
    `trait`) and `scripts/ingest_race_trait_generic.py` (`race_trait`) both
    write `<kind>_generic/` as a deliberate SIBLING to `<kind>/` -- never
    inside it, because existing curated consumers glob `<kind>/*.json`
    directly and would misinterpret the flatter generic-pass shape (both
    scripts' own docstrings state this). Their whole design leaned on the
    OLD kind-BLIND join counting a `<kind>_generic` record as a real answer
    for a `<kind>` census unit ("shape_ledger.py::build_corpus_index walks
    ... with no subdirectory-name filter, so a sibling directory is exactly
    as measurable for Gate-1 purposes" -- both scripts' own words). Making
    the join kind-AWARE (this fix) must preserve that intentional design,
    not silently break it -- so a trailing `_generic` suffix is stripped
    before the kind is used as (part of) an index/lookup key. This is the
    ONLY normalization applied: a genuinely different kind's directory
    (`ability` for a `trait` unit, `equipment` for an `equipment_modifier`
    unit, `race_trait_generic` for a `class_feature` unit) is never treated
    as equivalent -- those are exactly the real collisions this fix closes
    (see build_corpus_index's docstring for the corpus-wide re-derivation)."""
    if raw_kind.endswith(_GENERIC_KIND_DIR_SUFFIX):
        return raw_kind[: -len(_GENERIC_KIND_DIR_SUFFIX)]
    return raw_kind


# The ONE `equipment/<X>/` subdirectory name that denotes a DIFFERENT kind
# (`equipment_modifier`) rather than an ordinary `equipment`-kind
# sub-grouping. `equipment_gap.rs::generate()`'s own write path
# (`book_out.join("equipmods")` where `book_out = <book>/equipment`) is the
# ONLY place any `equipment_modifier` record is ever written -- confirmed
# corpus-wide, zero bare `equipment_modifier/` directories exist anywhere.
_EQUIPMENT_MODIFIER_SUBDIR = "equipmods"


def kind_from_path_parts(parts: list[str]) -> str:
    """Resolves the real `kind` for a corpus record from its path parts
    relative to the book directory (`parts[0]` is the top-level kind
    directory, `parts[1:]` are further nesting). Two normalizations, in
    order:

    1. `equipment/equipmods/...` -> `equipment_modifier` (see
       `_EQUIPMENT_MODIFIER_SUBDIR`'s docstring). A DIRECTORY-NAME check,
       not a blanket "always descend one more level" rule -- `arms_armor`/
       `general`/`magic_items` are ordinary `equipment`-kind sub-groupings
       under the SAME `equipment/<X>/` shape (confirmed corpus-wide: these
       are the only 4 subdirectory names that exist under any book's
       `equipment/`) and must stay `equipment`.
    2. `normalize_kind_dir` — strips a trailing `_generic` sibling suffix
       (`ingest_generic_kind.py`/`ingest_race_trait_generic.py`'s own
       deliberate design, see that function's docstring).

    `decisions.md §20` t9-onboarding straggler wave: making the join
    kind-aware (the concurrent `epic-6-kind-trait_cycle-2` fix) turned
    EVERY `equipment_modifier` record's real, universal directory
    convention into a false collision -- 1,003 units (essentially the
    entire kind) went `no_record` the moment the join started comparing
    `parts[0]` (`"equipment"`) against the unit's own `kind`
    (`"equipment_modifier"`) with no knowledge of this corpus's own nesting
    convention. This is the SAME shape `normalize_kind_dir`'s `_generic`
    suffix handling already exists to fix for other kinds -- extended here
    for the one kind whose real subdirectory convention is a NAME, not a
    suffix."""
    if len(parts) >= 2 and parts[0] == "equipment" and parts[1] == _EQUIPMENT_MODIFIER_SUBDIR:
        return "equipment_modifier"
    return normalize_kind_dir(parts[0])


def build_corpus_index(corpus_root: str, books: set[str] | None = None) -> dict:
    """Walks `data/corpus/<book>/<kind>/*.json` (excluding LICENSE.json) and
    indexes every record's raw_tokens by (book, kind, source_basename,
    source_line). `books`, if given, restricts the walk to those book
    directories (an optimisation; omit to index the whole corpus). A
    book in `BOOK_CORPUS_DIR_ALIASES` walks its aliased directory but is
    still indexed under its own (inventory-spelled) book name, so the
    join key `classify_unit` builds from a unit's own `book` field still
    matches. `kind` is the directory one level under the book directory
    the record physically lives in -- the exact same convention
    `build_corpus_key_index` already uses for its own fallback join.

    Returns {(book, kind, basename, line): [ {"key":..., "value":...}, ... ]}.
    A (book, kind, basename, line) with no DEFINE/BONUS tokens is still
    present in the index with an empty list, distinguishing "record found,
    no formula tokens" from "record not found" for the join_status field.

    KIND-AWARE BY CONSTRUCTION (`decisions.md §25` discovery-forward,
    `epic-6-kind-trait_cycle-2_cycle_receipt.md` §4): before this, the key
    omitted `kind`, so two different kinds' records that happened to cite
    the identical `(book, source_file, source_line)` LST coordinate --
    e.g. a pre-`Kind::Trait` generic-ingest pass writing a `kind: ability`
    record at the same coordinate a `kind: trait` census unit cites --
    collided in this dict (last-glob-order write silently won), and a
    unit's join could be answered by the WRONG kind's record.
    `normalize_kind_dir` (see its own docstring) means a `<kind>_generic`
    sibling directory (the deliberate design `ingest_generic_kind.py`/
    `ingest_race_trait_generic.py` already rely on) still counts as a real
    answer for its base kind -- so `race_trait`/`race_trait_generic`,
    `feat`/`feat_generic`, `race`/`race_generic`, `monster`/`monster_
    generic`, `class`/`class_generic` are NOT collisions this fix reports.
    Re-derived corpus-wide, EXCLUDING those intentional pairs (this
    cycle's own receipt): **1,511** not-done units across 3 kind-pairs
    currently join to a genuinely different kind's record under the old
    key shape -- `equipment_modifier`->`equipment` 999, `trait`->`ability`
    487, `class_feature`->`race_trait` (via its `race_trait_generic`
    sibling) 25 -- command: `python3 <repo-scratch>/blast_radius2.py`
    (script logic inlined in the cycle receipt this fix cites)."""
    index: dict[tuple[str, str, str, int], list[dict]] = {}
    if books is not None:
        search_roots = []
        for b in sorted(books):
            aliased = BOOK_CORPUS_DIR_ALIASES.get(b)
            if aliased and aliased != b:
                # An aliased book does not necessarily keep EVERY kind under
                # the legacy directory -- e.g. `bestiary`'s `monster_ability`
                # records live under the legacy `beastiary/` spelling, but
                # its `spell`/`equipment` records live under the correctly
                # spelled `bestiary/`. Walk both and merge; a real key can
                # only be minted once (one (book, source_file, source_line)
                # per corpus record) so there is no collision to resolve.
                search_roots.append((b, os.path.join(corpus_root, aliased)))
                search_roots.append((b, os.path.join(corpus_root, b)))
            else:
                search_roots.append((b, os.path.join(corpus_root, b)))
    else:
        search_roots = [(None, corpus_root)]
    for book_override, root in search_roots:
        if not os.path.isdir(root):
            continue
        book = book_override if book_override is not None else os.path.relpath(root, corpus_root).split(os.sep)[0]
        for path in glob.glob(os.path.join(root, "**", "*.json"), recursive=True):
            if os.path.basename(path) == "LICENSE.json":
                continue
            try:
                with open(path, "r", encoding="utf-8") as fh:
                    rec = json.load(fh)
            except (OSError, json.JSONDecodeError):
                continue
            source = rec.get("source") or {}
            src_path = source.get("path")
            src_line = source.get("line")
            if not src_path or src_line is None:
                continue
            basename = os.path.basename(src_path)
            # kind = the directory one level under the book directory this
            # record physically lives in -- same derivation
            # build_corpus_key_index already uses, so both indexes agree on
            # what "kind" means for the same record.
            rel = os.path.relpath(path, root)
            parts = rel.split(os.sep)
            if len(parts) < 2:
                continue
            kind = kind_from_path_parts(parts)
            raw_tokens = (rec.get("data") or {}).get("raw_tokens") or []
            formula_tokens = [
                t
                for t in raw_tokens
                if isinstance(t, dict)
                and isinstance(t.get("key"), str)
                and (t["key"] == "DEFINE" or t["key"].startswith("BONUS"))
            ]
            index[(book, kind, basename, src_line)] = formula_tokens
    return index


def build_corpus_key_index(corpus_root: str, books: set[str] | None = None) -> dict:
    """Walks `data/corpus/<book>/<kind>/*.json` (excluding LICENSE.json) and
    indexes every record's `DEFINE`/`BONUS*` formula tokens by
    `(book, kind, data.key)` -- the record's own declared identity, never
    its `name` (`gen_equipment_gap_tables.rs`'s `held` map already
    documents the name-collision hazard: an early version inserted display
    names too and silently suppressed 28 unrelated records that happened
    to share one).

    This is the FALLBACK join `classify_unit` consults only when the
    primary `(book, source_file, source_line)` join misses. `decisions.md
    §20`/§17a: `ultimate_magic`'s 11 `equipment` units traced this cycle
    are not a gap at all -- `docs/work-inventory.json`'s own equipment
    enumeration mints exactly one unit per corpus_key, and for several
    `.COPY=`-aliased spellbooks that key's surviving unit cites
    `_pfs/pfs_um_equip_general.lst`'s PFS-legality-overlay row (no new
    content, just a `TYPE:PFSNotLegal` restatement of an existing item)
    rather than the base `um_equip_general.lst` row the real, already-
    ingested corpus record was actually generated from and cites. The
    primary join is exactly right to fail on that mismatched citation --
    the record it names truly was never written -- but the corpus record
    for the SAME key, cited from the OTHER file, already exists. This
    index recovers that: a corpus_key match within the same (book, kind)
    is the record, addressed by its own real identity rather than by the
    LST row this particular inventory unit happened to be minted from.

    Returns {(book, kind, key): [ {"key":..., "value":...}, ... ]} -- same
    value shape as `build_corpus_index`, so `classify_unit` can classify a
    fallback hit exactly like a primary one (including `no_formula_tokens`
    when the record legitimately carries none, which is the TRUE
    classification for a pregenerated spellbook: real record, no BONUS/
    DEFINE token, not a gap in our work)."""
    index: dict[tuple[str, str, str], list[dict]] = {}
    if books is not None:
        search_roots = []
        for b in sorted(books):
            aliased = BOOK_CORPUS_DIR_ALIASES.get(b)
            if aliased and aliased != b:
                search_roots.append((b, os.path.join(corpus_root, aliased)))
                search_roots.append((b, os.path.join(corpus_root, b)))
            else:
                search_roots.append((b, os.path.join(corpus_root, b)))
    else:
        search_roots = [(None, corpus_root)]
    for book_override, root in search_roots:
        if not os.path.isdir(root):
            continue
        book = book_override if book_override is not None else os.path.relpath(root, corpus_root).split(os.sep)[0]
        for path in glob.glob(os.path.join(root, "**", "*.json"), recursive=True):
            if os.path.basename(path) == "LICENSE.json":
                continue
            try:
                with open(path, "r", encoding="utf-8") as fh:
                    rec = json.load(fh)
            except (OSError, json.JSONDecodeError):
                continue
            data = rec.get("data") or {}
            record_key = data.get("key")
            if not isinstance(record_key, str) or not record_key:
                continue
            # kind = the directory one level under the book directory this
            # record physically lives in (e.g. `.../ultimate_magic/equipment/
            # book_of_harms.json` -> kind "equipment") -- never guessed from
            # the inventory unit's own `kind` field, so a book-kind
            # directory the corpus does not actually carry can never mint a
            # phantom index entry. `normalize_kind_dir` strips a `_generic`
            # sibling suffix (see its own docstring) so this fallback index
            # honors the same intentional `<kind>_generic` design the
            # primary `build_corpus_index` join does.
            rel = os.path.relpath(path, root)
            parts = rel.split(os.sep)
            if len(parts) < 2:
                continue
            kind = kind_from_path_parts(parts)
            raw_tokens = data.get("raw_tokens") or []
            formula_tokens = [
                t
                for t in raw_tokens
                if isinstance(t, dict)
                and isinstance(t.get("key"), str)
                and (t["key"] == "DEFINE" or t["key"].startswith("BONUS"))
            ]
            index[(book, kind, record_key)] = formula_tokens
    return index


def build_cross_book_key_index(corpus_root: str, books: set[str] | None = None) -> dict:
    """Walks `data/corpus/<book>/<kind>/*.json` and indexes every record's
    `DEFINE`/`BONUS*` formula tokens by `(kind, data.key)` alone -- no book
    in the key at all. This is the THIRD, coarsest fallback `classify_unit`
    consults only when BOTH the primary `(book, source_file, source_line)`
    join and the same-book `(book, kind, key)` fallback (`build_corpus_key_
    index`) miss.

    `decisions.md §20` t9-onboarding straggler wave: a book's row can
    deliberately NOT (re-)declare a record that already exists under a
    DIFFERENT book entirely -- `ingest_spells.rs`'s own `already_ingested_
    oa`/`already_ingested_uc` is the documented, already-shipped example
    ("a handful of rows exist only to widen an existing spell's class
    access"; `occult_adventures:spell:repulsion` cites `oa_spells.lst:464`
    but "Repulsion" is never re-ingested for `occult_adventures` -- the
    real record ships under `crb`'s own citation entirely). The same shape
    recurs for `ability` (a book's `.MOD` PFS-legality overlay restricting
    a trait originally declared, and already ingested, under a DIFFERENT
    book) and `equipment_modifier`. Neither the primary join (different
    file) nor the same-book key fallback (different book) can ever resolve
    these; only a book-agnostic `(kind, key)` lookup can.

    A PCGen `KEY:` token is meant to be a globally unique identifier within
    its category, so a same-key match across two DIFFERENT books' records
    for the SAME kind is expected to be the literal same game object, not a
    coincidence -- unlike `data.name` (`gen_equipment_gap_tables.rs`'s
    `held`-map name-collision hazard this module's `build_corpus_key_index`
    already documents), which is a display string with no such uniqueness
    guarantee. Still, "expected" is not "proven": when two DIFFERENT books'
    records under the identical `(kind, key)` carry DIFFERENT formula
    tokens, this is recorded as an explicit `None` -- an ambiguous
    collision the fallback must decline rather than guess at
    (`decisions.md §1a`: under-include rather than invent). Two books
    legitimately restating the SAME record (identical formula tokens) is
    not that hazard and resolves normally.

    Returns `{(kind, key): (book, formula_tokens) | None}` -- `None` marks
    an ambiguous collision; a present `(book, formula_tokens)` tuple names
    which book's record the fallback would actually attribute the match to
    (for receipts/audits), same value shape as `build_corpus_key_index`
    otherwise."""
    # (kind, key) -> {book: formula_tokens} while building, so a genuine
    # collision (two DIFFERENT books, DIFFERENT tokens) can be detected
    # before collapsing to the public return shape.
    seen: dict[tuple[str, str], dict[str, list[dict]]] = {}
    if books is not None:
        search_roots = []
        for b in sorted(books):
            aliased = BOOK_CORPUS_DIR_ALIASES.get(b)
            if aliased and aliased != b:
                search_roots.append((b, os.path.join(corpus_root, aliased)))
                search_roots.append((b, os.path.join(corpus_root, b)))
            else:
                search_roots.append((b, os.path.join(corpus_root, b)))
    else:
        search_roots = [(None, corpus_root)]
    for book_override, root in search_roots:
        if not os.path.isdir(root):
            continue
        for path in glob.glob(os.path.join(root, "**", "*.json"), recursive=True):
            if os.path.basename(path) == "LICENSE.json":
                continue
            try:
                with open(path, "r", encoding="utf-8") as fh:
                    rec = json.load(fh)
            except (OSError, json.JSONDecodeError):
                continue
            data = rec.get("data") or {}
            record_key = data.get("key")
            if not isinstance(record_key, str) or not record_key:
                continue
            rel = os.path.relpath(path, root)
            parts = rel.split(os.sep)
            if len(parts) < 2:
                continue
            # When `books` is given, `root` is already the single book's own
            # directory, so `parts[0]` is the KIND subdirectory and `book`
            # is the known override. When `books` is None (whole-corpus
            # walk), `root` is `corpus_root` itself, so `parts[0]` is the
            # BOOK directory and the kind is `parts[1]`.
            if book_override is not None:
                book = book_override
                kind = kind_from_path_parts(parts)
            else:
                book = parts[0]
                if len(parts) < 3:
                    continue
                kind = kind_from_path_parts(parts[1:])
            raw_tokens = data.get("raw_tokens") or []
            formula_tokens = [
                t
                for t in raw_tokens
                if isinstance(t, dict)
                and isinstance(t.get("key"), str)
                and (t["key"] == "DEFINE" or t["key"].startswith("BONUS"))
            ]
            seen.setdefault((kind, record_key), {})[book] = formula_tokens

    index: dict[tuple[str, str], tuple[str, list[dict]] | None] = {}
    for (kind, key), by_book in seen.items():
        books_here = list(by_book.items())
        if len(books_here) == 1:
            index[(kind, key)] = books_here[0]
            continue
        # Multiple books share this (kind, key). Not ambiguous if every
        # book's formula tokens are identical (a legitimate restatement);
        # ambiguous the moment any two diverge.
        first_book, first_tokens = books_here[0]
        if all(tokens == first_tokens for _b, tokens in books_here[1:]):
            index[(kind, key)] = (first_book, first_tokens)
        else:
            index[(kind, key)] = None
    return index


def extract_formula_segment(key: str, value: str) -> str | None:
    """Extracts the formula-bearing segment from one DEFINE/BONUS raw
    token, per this module's docstring 'Extraction method'.

    DEFINE format: `VarName|InitialValueOrFormula` -> field 1.
    BONUS:* format: `SubType|Target|Formula|...extra` -> field 2 (0-based),
    the field every observed BONUS subtype (VAR, SKILL, STAT, COMBAT, ...)
    carries its formula/magnitude in, per this script's corpus sampling
    (module docstring). A token with fewer fields than its format expects
    yields None -- callers must not treat that as a match against any
    family.
    """
    parts = value.split("|")
    if key == "DEFINE":
        return parts[1] if len(parts) >= 2 else None
    if key.startswith("BONUS"):
        if len(parts) >= 3:
            return parts[2]
        # T9-onboarding-cause-closure (2026-08-23, row 17's remaining 21):
        # `ultimate_campaign:trait:trait_harvester`'s `BONUS:SKILL|%LIST` --
        # oracle-verified to be the ONLY occurrence of this exact 2-field
        # shorthand anywhere in the pinned corpus
        # (`uca_abilities_traits.lst:198`; a corpus-wide scan of every
        # `BONUS:` token found no second instance of a `SKILL` subtype with
        # `%LIST` as its 2nd field and no 3rd field). PCGen omits the
        # magnitude field entirely for this CHOOSE:SKILL-linked flat trait
        # bonus shape; the record's own `ASPECT` token ("+1 trait bonus on
        # Profession (tanner) or Profession (trapper) checks...") and `DESC`
        # confirm the omitted magnitude is a flat +1, matching Pathfinder's
        # universal trait skill-bonus convention this shorthand always
        # implies -- never guessed, always derived from the record's own
        # declared text. Scoped to the `%LIST` shorthand specifically (never
        # a bare `SKILL|<real skill name>` with a missing magnitude, which
        # stays the genuine parse failure it already was).
        if len(parts) == 2 and parts[0] == "SKILL" and parts[1] == "%LIST":
            return "1"
        return None
    return None


def classify_formula(formula: str) -> str:
    """Classifies one formula segment into its primary family id, per
    FAMILIES' priority order. Always returns a family id -- falls through
    to FAMILY_F8_OTHER rather than ever returning None, so a caller never
    needs its own "unclassified" branch for a non-empty formula."""
    stripped = formula.strip()
    if not stripped:
        return FAMILY_F0_NO_FORMULA
    for family_id, _label, predicate, _proof_width in FAMILIES:
        if predicate(stripped):
            return family_id
    return FAMILY_F8_OTHER


def classify_unit(
    unit: dict,
    corpus_index: dict,
    key_index: dict | None = None,
    cross_book_key_index: dict | None = None,
) -> dict:
    """Classifies one not-done unit. Returns a ledger row dict with the
    unit's id/kind/book, its `family` (always non-null -- see
    classify_formula/FAMILY_F0_NO_FORMULA), and `join_status` so a
    reviewer can tell "genuinely no formula" apart from "join failed".

    `key_index` (`build_corpus_key_index`'s output, `(book, kind, key) ->
    formula_tokens`) is a FALLBACK consulted only when the primary
    `(book, source_file, source_line)` join misses -- never in place of
    it. `decisions.md §20`/§17a: a unit whose inventory citation points at
    a content-free legality-overlay row can still have a real, already-
    ingested corpus record under its own `corpus_key`, cited from a
    DIFFERENT physical file the primary join never looks at. Omitting
    `key_index` (or passing `None`) keeps every existing caller's behavior
    byte-for-byte unchanged.

    `cross_book_key_index` (`build_cross_book_key_index`'s output, `(kind,
    key) -> (book, formula_tokens) | None`) is a THIRD, coarsest fallback
    consulted only when BOTH the primary join and `key_index` miss -- the
    shape where a record is already ingested under a DIFFERENT book than
    the one this unit's row happens to live in (see that function's
    docstring). A `None` value marks an ambiguous cross-book key collision
    and is never treated as a match."""
    book = unit.get("book")
    kind = unit.get("kind")
    basename = unit.get("source_file")
    line = unit.get("source_line")
    # KIND-AWARE: the key includes the unit's own `kind` so a different
    # kind's record sitting at the identical (book, source_file,
    # source_line) LST coordinate can never answer this unit's join (see
    # build_corpus_index's docstring for the real-world collision this
    # closes).
    key = (book, kind, basename, line) if (book and kind and basename and line is not None) else None
    formula_tokens = corpus_index.get(key) if key is not None else None

    if formula_tokens is None and key_index:
        kind = unit.get("kind")
        corpus_key = unit.get("corpus_key")
        if book and kind and corpus_key:
            formula_tokens = key_index.get((book, kind, corpus_key))

    if formula_tokens is None and cross_book_key_index:
        kind = unit.get("kind")
        corpus_key = unit.get("corpus_key")
        if kind and corpus_key:
            hit = cross_book_key_index.get((kind, corpus_key))
            if hit is not None:
                _matched_book, formula_tokens = hit

    pi_redacted_formula = False
    if formula_tokens is None:
        join_status = "no_record"
        family = FAMILY_F0_NO_FORMULA
    elif not formula_tokens:
        join_status = "no_formula_tokens"
        family = FAMILY_F0_NO_FORMULA
    else:
        join_status = "matched"
        # A token's VALUE can itself be the PI-redaction marker (the
        # record's mechanical BONUS/DEFINE value was blanket-redacted
        # alongside its NAME/DESC, not because the formula has no shape --
        # `decisions.md §24b` names the record as renamed, never as
        # formula-empty). Detected here, on the SAME formula_tokens the
        # classifier below reads, so the two can never disagree.
        pi_redacted_formula = any(PS.REDACTED_PI_MARKER in (tok.get("value") or "") for tok in formula_tokens)
        # Primary family: classify every DEFINE/BONUS formula segment on
        # this unit's record, then take the highest-priority (lowest
        # FAMILIES-index) family any segment produced -- mirroring
        # MEASURE-TWICE.md §3's "PRIMARY family assigned by priority
        # order when [a unit] carries more than one shape".
        priority = {fid: i for i, (fid, *_rest) in enumerate(FAMILIES)}
        best_family = None
        best_rank = None
        for tok in formula_tokens:
            seg = extract_formula_segment(tok["key"], tok["value"])
            if seg is None:
                continue
            fam = classify_formula(seg)
            rank = priority.get(fam, len(FAMILIES) + 1)  # F0/F8 sort last
            if best_rank is None or rank < best_rank:
                best_rank = rank
                best_family = fam
        family = best_family if best_family is not None else FAMILY_F0_NO_FORMULA

    # `decisions.md §27a`/row-17 instrument: WITHIN family F0, distinguish
    # a genuinely-derived answer from a placeholder wearing F0's label.
    # `join_status` alone does not do this -- `matched` already collapses
    # "real family found" and "every present token still fell through to
    # F0" into one bucket, which is exactly the conflation §27a calls out.
    #   - "engine_does_not_hold"   : join found no corpus record at all (join_status
    #                        "no_record"). Not row-17's population yet --
    #                        `decisions.md §27`/`kanban.md` row 17 sequences
    #                        AFTER `no_record` reaches zero.
    #   - "measured_empty" : a real corpus record was found and it
    #                        genuinely carries zero DEFINE/BONUS tokens
    #                        (join_status "no_formula_tokens"). This is F0
    #                        "reached by measurement" -- a real answer.
    #   - "measured_pi_redacted" : the join succeeded, and EVERY DEFINE/
    #                        BONUS token that failed to classify did so
    #                        because its value IS the PI-redaction marker
    #                        (`pi_redacted_formula`), never a malformed/
    #                        short token. T9-onboarding-cause-closure
    #                        (2026-08-23, row 17's remaining 21):
    #                        `decisions.md §27a` -- "if the value genuinely
    #                        carries PI, it stays redacted -- but then it is
    #                        not a fallthrough placeholder, it is a
    #                        correctly-measured redacted value." Reached
    #                        ONLY for records that survived
    #                        `pi_scrub.py::scrub_name_pi_tokens`'s narrower
    #                        (`neutral_name`-driven) redaction and are
    #                        STILL fully wiped -- i.e. a genuine blacklist
    #                        hit or a normalized-only identity hit, never a
    #                        merely-unnarrowed self-reference. This is a
    #                        REAL answer (the shape is "PI, cannot be
    #                        shipped as formula"), not row 17's placeholder
    #                        population.
    #   - "fallthrough"    : the join succeeded AND the record carries
    #                        DEFINE/BONUS tokens, but every one of them
    #                        failed to classify into a real family for a
    #                        reason OTHER than PI redaction (a malformed/
    #                        short token). This is F0 "reached by
    #                        nothing else matched" -- exactly the
    #                        placeholder §27a names, and row 17's real
    #                        population.
    if family == FAMILY_F0_NO_FORMULA:
        if join_status == "no_record":
            f0_reached_by = "engine_does_not_hold"
        elif join_status == "no_formula_tokens":
            f0_reached_by = "measured_empty"
        elif pi_redacted_formula:
            f0_reached_by = "measured_pi_redacted"
        else:
            f0_reached_by = "fallthrough"
    else:
        f0_reached_by = None

    return {
        "id": unit.get("id"),
        "kind": unit.get("kind"),
        "book": book,
        "family": family,
        "join_status": join_status,
        "f0_reached_by": f0_reached_by,
        "pi_redacted_formula": pi_redacted_formula,
    }


def _family_metadata() -> dict:
    meta = {
        FAMILY_F0_NO_FORMULA: {"label": _FAMILY_F0_LABEL, "proof_width": _FAMILY_F0_PROOF_WIDTH},
        FAMILY_F8_OTHER: {"label": _FAMILY_F8_LABEL, "proof_width": _FAMILY_F8_PROOF_WIDTH},
    }
    for fid, label, _predicate, proof_width in FAMILIES:
        meta[fid] = {"label": label, "proof_width": proof_width}
    return meta


def build_ledger(
    units: list[dict],
    corpus_index: dict,
    key_index: dict | None = None,
    cross_book_key_index: dict | None = None,
) -> dict:
    """Returns {'population', 'rows', 'families', 'unclassified_count',
    'join_status_counts'}. `unclassified_count` is structurally always 0
    for a non-empty population -- classify_unit never returns a null
    family -- so its presence at 0 is the AT-32-G1-001/002 gate variable,
    not a tautology this script could accidentally satisfy: it is 0
    because F0/F8 exist as real, counted, honestly-labelled families, not
    because nothing was checked.

    `join_status_counts` (decisions.md §14b) is the honest split that
    `unclassified_count`/`families` alone hides: F0 (20,113 of 24,914,
    81%) conflates two different populations its own proof-width text
    distinguishes -- `no_record` ("the join found nothing", 10,419) vs.
    `no_formula_tokens` ("found a record, it carries no DEFINE/BONUS",
    9,694) -- plus `matched` (4,801, the only status resting on a real
    corpus record). A quoted `unclassified_count: 0` without this split
    is a bare total under `decisions.md §12c`; this field is what every
    caller (this script's own printed output, the standing gate, the
    family-vocabulary reconciliation, and every AT-32-G1-* criterion that
    quotes a coverage figure) must surface alongside it."""
    rows = [classify_unit(u, corpus_index, key_index, cross_book_key_index) for u in units]
    meta = _family_metadata()
    counts: dict[str, int] = {}
    join_status_counts: dict[str, int] = {}
    # decisions.md §27a / row 17: F0's own three-way split (engine_does_not_hold /
    # measured_empty / fallthrough), plus the PI-redaction sub-count within
    # fallthrough -- see classify_unit's docstring for what each means.
    f0_breakdown: dict[str, int] = {}
    f0_fallthrough_pi_redacted = 0
    for row in rows:
        counts[row["family"]] = counts.get(row["family"], 0) + 1
        status = row.get("join_status") or "unknown"
        join_status_counts[status] = join_status_counts.get(status, 0) + 1
        reached_by = row.get("f0_reached_by")
        if reached_by:
            f0_breakdown[reached_by] = f0_breakdown.get(reached_by, 0) + 1
            if reached_by == "fallthrough" and row.get("pi_redacted_formula"):
                f0_fallthrough_pi_redacted += 1
    families = {
        fid: {
            "label": meta.get(fid, {}).get("label", ""),
            "proof_width": meta.get(fid, {}).get("proof_width", ""),
            "count": count,
        }
        for fid, count in counts.items()
    }
    unclassified = [r for r in rows if not r.get("family")]
    return {
        "population": len(units),
        "rows": rows,
        "families": families,
        "unclassified_count": len(unclassified),
        "unclassified": [r["id"] for r in unclassified],
        "join_status_counts": join_status_counts,
        "f0_breakdown": f0_breakdown,
        "f0_fallthrough_pi_redacted": f0_fallthrough_pi_redacted,
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--inventory", default=DEFAULT_INVENTORY, help="path to docs/work-inventory.json")
    parser.add_argument("--corpus-root", default=DEFAULT_CORPUS_ROOT, help="path to data/corpus")
    parser.add_argument("--output", help="write the full ledger as JSON to this path")
    args = parser.parse_args(argv)

    try:
        inventory = load_inventory_or_die(args.inventory)
    except (OSError, json.JSONDecodeError) as exc:
        print(f"no coverage: cannot read/parse inventory at {args.inventory!r}: {exc}", file=sys.stderr)
        return 1

    units = CL.not_done_population(inventory)
    if not units:
        print(
            f"no coverage: inventory at {args.inventory!r} has zero not-done units; "
            "an empty population cannot manufacture a passing ledger (fail-closed posture, AT-32-G1-002)",
            file=sys.stderr,
        )
        return 1

    books = {u.get("book") for u in units if u.get("book")}
    corpus_index = build_corpus_index(args.corpus_root, books)
    key_index = build_corpus_key_index(args.corpus_root, books)
    # Whole-corpus walk (no `books` restriction): the cross-book fallback's
    # entire purpose is finding a record ingested under a DIFFERENT book
    # than any of the not-done units' own books, so restricting the walk to
    # `books` would make it structurally unable to ever find anything.
    cross_book_key_index = build_cross_book_key_index(args.corpus_root)
    ledger = build_ledger(units, corpus_index, key_index, cross_book_key_index)

    print(f"population (not-done units considered): {ledger['population']}")
    print(f"unclassified: {ledger['unclassified_count']}")
    # decisions.md §14b: a bare unclassified_count/family total hides that
    # only `matched` rests on a real corpus record -- `no_record` and
    # `no_formula_tokens` both collapse into F0 above but mean different
    # things (join found nothing vs. join found a record with no formula).
    jsc = ledger.get("join_status_counts", {})
    matched = jsc.get("matched", 0)
    no_record = jsc.get("no_record", 0)
    no_formula_tokens = jsc.get("no_formula_tokens", 0)
    pop = ledger["population"] or 1
    print(
        "join-status split (decisions.md §14b -- this is the honest coverage "
        "figure, not the family rollup below on its own):"
    )
    print(f"  matched            {matched:>7}  ({100.0 * matched / pop:.1f}%)  -- rests on a real corpus record")
    print(f"  no_formula_tokens  {no_formula_tokens:>7}  ({100.0 * no_formula_tokens / pop:.1f}%)  -- record found, carries no DEFINE/BONUS")
    print(f"  no_record          {no_record:>7}  ({100.0 * no_record / pop:.1f}%)  -- join found no corpus record at all")
    print()
    # decisions.md §27a / kanban.md row 17: F0's own breakdown -- how much
    # of F0 is a real measurement vs. a placeholder wearing F0's label.
    f0b = ledger.get("f0_breakdown", {})
    f0_total = ledger["families"].get(FAMILY_F0_NO_FORMULA, {}).get("count", 0)
    fallthrough = f0b.get("fallthrough", 0)
    print(f"F0 breakdown ({f0_total} total, decisions.md §27a -- row 17's real population lives here):")
    print(f"  engine_does_not_hold       {f0b.get('engine_does_not_hold', 0):>7}  -- no corpus record yet; not row 17's population (sequenced after no_record==0)")
    print(f"  measured_empty     {f0b.get('measured_empty', 0):>7}  -- real corpus record, genuinely zero DEFINE/BONUS tokens (F0 by measurement)")
    print(f"  fallthrough        {fallthrough:>7}  -- corpus record matched with formula tokens present, but every one failed to classify (F0 by 'nothing else matched' -- row 17's population)")
    print(f"    of which PI-redacted formula value: {ledger.get('f0_fallthrough_pi_redacted', 0)}")
    print()
    print("family rollup:")
    for fid, info in sorted(ledger["families"].items()):
        print(f"  {fid:<4} {info['count']:>7}  {info['label']}")

    if args.output:
        os.makedirs(os.path.dirname(args.output) or ".", exist_ok=True)
        with open(args.output, "w", encoding="utf-8") as fh:
            json.dump(ledger, fh, indent=2)
            fh.write("\n")

    return 0


if __name__ == "__main__":
    sys.exit(main())
