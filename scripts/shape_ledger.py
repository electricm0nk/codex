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


def build_corpus_index(corpus_root: str, books: set[str] | None = None) -> dict:
    """Walks `data/corpus/<book>/**/*.json` (excluding LICENSE.json) and
    indexes every record's raw_tokens by (book, source_basename,
    source_line). `books`, if given, restricts the walk to those book
    directories (an optimisation; omit to index the whole corpus). A
    book in `BOOK_CORPUS_DIR_ALIASES` walks its aliased directory but is
    still indexed under its own (inventory-spelled) book name, so the
    join key `classify_unit` builds from a unit's own `book` field still
    matches.

    Returns {(book, basename, line): [ {"key":..., "value":...}, ... ]}.
    A (book, basename, line) with no DEFINE/BONUS tokens is still present
    in the index with an empty list, distinguishing "record found, no
    formula tokens" from "record not found" for the join_status field.
    """
    index: dict[tuple[str, str, int], list[dict]] = {}
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
            raw_tokens = (rec.get("data") or {}).get("raw_tokens") or []
            formula_tokens = [
                t
                for t in raw_tokens
                if isinstance(t, dict)
                and isinstance(t.get("key"), str)
                and (t["key"] == "DEFINE" or t["key"].startswith("BONUS"))
            ]
            index[(book, basename, src_line)] = formula_tokens
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
        return parts[2] if len(parts) >= 3 else None
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


def classify_unit(unit: dict, corpus_index: dict) -> dict:
    """Classifies one not-done unit. Returns a ledger row dict with the
    unit's id/kind/book, its `family` (always non-null -- see
    classify_formula/FAMILY_F0_NO_FORMULA), and `join_status` so a
    reviewer can tell "genuinely no formula" apart from "join failed"."""
    book = unit.get("book")
    basename = unit.get("source_file")
    line = unit.get("source_line")
    key = (book, basename, line) if (book and basename and line is not None) else None
    formula_tokens = corpus_index.get(key) if key is not None else None

    if formula_tokens is None:
        join_status = "no_record"
        family = FAMILY_F0_NO_FORMULA
    elif not formula_tokens:
        join_status = "no_formula_tokens"
        family = FAMILY_F0_NO_FORMULA
    else:
        join_status = "matched"
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

    return {
        "id": unit.get("id"),
        "kind": unit.get("kind"),
        "book": book,
        "family": family,
        "join_status": join_status,
    }


def _family_metadata() -> dict:
    meta = {
        FAMILY_F0_NO_FORMULA: {"label": _FAMILY_F0_LABEL, "proof_width": _FAMILY_F0_PROOF_WIDTH},
        FAMILY_F8_OTHER: {"label": _FAMILY_F8_LABEL, "proof_width": _FAMILY_F8_PROOF_WIDTH},
    }
    for fid, label, _predicate, proof_width in FAMILIES:
        meta[fid] = {"label": label, "proof_width": proof_width}
    return meta


def build_ledger(units: list[dict], corpus_index: dict) -> dict:
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
    rows = [classify_unit(u, corpus_index) for u in units]
    meta = _family_metadata()
    counts: dict[str, int] = {}
    join_status_counts: dict[str, int] = {}
    for row in rows:
        counts[row["family"]] = counts.get(row["family"], 0) + 1
        status = row.get("join_status") or "unknown"
        join_status_counts[status] = join_status_counts.get(status, 0) + 1
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
    ledger = build_ledger(units, corpus_index)

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
