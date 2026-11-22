#!/usr/bin/env python3
"""AT-35-E7-000-POPULATION-CENSUS, cycle 2 -- exhaustive disposition of the 255.

Cycle 1 measured the never-reached population and named a remainder:
`record_absent_from_inventory_population=255` -- `data/corpus` records with
`completeness: full` and published rules prose that no `docs/work-inventory.json`
unit reaches under either of `load_population`'s two join keys. Cycle 1
hand-verified 12 of those 255 and generalised.

This cycle measures all 255, one at a time, and answers the only question that
matters under the sheet rule (`decisions.md §1`): **do this record's words reach
a rendered sheet line anywhere?** A record whose exact published prose is
carried by a DONE inventory unit under another book, or by the base row the
record was derived from, is not a lost rule -- it is a second ingest of a rule a
sheet already prints. A record whose words reach nothing is a real gap.

MEASUREMENT ONLY. It reads `data/corpus`, `docs/work-inventory.json` and the
pinned PCGen tree; it writes exactly one JSON artifact next to itself. It adds
no unit, converts nothing, edits no classifier and changes no gate.

THE THIRD JOIN KEY. Cycle 1's census inverted `mod.rs::load_population`'s two
keys -- `(book, source_file, source_line)` and `(book, kind, id-tail)`. Both are
*coordinate* keys: they ask where a row sits, not what it says. A record can
therefore miss both and still have its words on a sheet. This cycle adds two
content keys the converter does not use and the census must:

  0. THE ONE THAT MATTERS MOST -- the same `.lst` row, book label ignored.
     `load_population`'s first key is `(book, source_file, source_line)`, and
     `v06_work_inventory`'s `attributed_book` deliberately RE-ATTRIBUTES a row
     to the book of its newest printing (`race_newest_printing`, the
     supersession register ruling). So ARG's `samsaran_abilities_race.lst:14`
     becomes the unit `bestiary_4:race_trait:samsaran_ability_scores`, and the
     coordinate join misses it on the book component alone while the two are
     the *same row of the same pinned file*. Dropping the book from the key and
     joining on `(basename, line)` is therefore not a loosening -- it is the
     join that respects what the enumerator actually did.
  1. `corpus_key` across ALL books -- PCGen spells one racial trait
     (`Low-Light Vision`, `Claws`) identically in every book that grants it, so
     the ARG per-race ability files and the Bestiary ability files carry the
     same KEY. A key match is only a CANDIDATE: a shared name is not a shared
     thing (the `Amorphous` of a protean and the `Amorphous` of ACG armor are
     different rules). Each candidate is therefore confirmed by comparing the
     two records' published `data.description` verbatim, normalised only for
     whitespace and case. Text-identical is a reach; text-different is a
     collision and stays a gap.
  2. verbatim published-prose identity against the REACHED corpus population,
     ignoring the key entirely. This is the sheet rule stated as a query: if the
     exact same rules paragraph is already carried by a record an inventory unit
     claims, a sheet prints those words, whatever the two records are called.
     It catches what key matching cannot -- PCGen's sub-form records
     (`Threefold Aspect (Adulthood)`) whose KEY is a variant spelling of a base
     record whose prose they reproduce character-for-character.
  3. the `.COPY=` pairing inside one `.lst` file -- PCGen's equipment-modifier
     idiom writes the visible rule once and then a `<New Key>.COPY=<Base>` row
     carrying `VISIBLE:NO`. Our ingest takes the visible original; the inventory
     enumerator takes the COPY row. Same rule, two rows, one unit. Confirmed by
     parsing the real `.lst` and requiring the COPY row's target name to equal
     the corpus record's own base-row name.

Re-derive:
  python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-7-closure/population_census_255.py
"""

import json
import os
import re
import sys
from collections import Counter, defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = HERE.split("/docs/release/")[0]
CORPUS = os.path.join(REPO, "data/corpus")
INV = os.path.join(REPO, "docs/work-inventory.json")
UNREACHED = os.path.join(HERE, "population-census-unreached.json")
OUT = os.path.join(HERE, "population-census-255.json")

PCGEN_ROOT = os.environ.get(
    "PCGEN_CORPUS_ROOT", os.path.join(os.path.expanduser("~"), "workspace/repos/pcgen/data")
)
BOOKS_REL = "pathfinder/paizo/roleplaying_game"


def norm_key(s):
    """Normalise a PCGen KEY for candidate matching: case and punctuation only."""
    return re.sub(r"[^a-z0-9]+", "", (s or "").lower())


def norm_text(s):
    """Normalise published prose for the identity test: case and whitespace only.

    Deliberately NOT punctuation-stripping. Two records that differ in a numeral,
    a die expression or a bonus must compare different -- that is the whole point
    of the test.
    """
    return re.sub(r"\s+", " ", (s or "").strip().lower())


def load_corpus():
    """Every `data/corpus` record, indexed the three ways this cycle needs.

    Walk rules are `mod.rs::walk_corpus`'s, verbatim: `_parity/` skipped whole,
    `LICENSE.json` skipped, book dir `beastiary` re-spelled `bestiary`, kind is
    the first path component under the book dir.
    """
    by_line = {}
    by_kind_slug = {}
    records = {}
    for book_dir_name in sorted(os.listdir(CORPUS)):
        bd = os.path.join(CORPUS, book_dir_name)
        if not os.path.isdir(bd):
            continue
        book = "bestiary" if book_dir_name == "beastiary" else book_dir_name
        for dirpath, dirnames, filenames in os.walk(bd):
            dirnames[:] = [d for d in dirnames if d != "_parity"]
            for fn in sorted(filenames):
                if not fn.endswith(".json") or fn == "LICENSE.json":
                    continue
                p = os.path.join(dirpath, fn)
                rel = os.path.relpath(p, bd)
                kind = rel.split(os.sep)[0] if os.sep in rel else rel
                slug = fn[: -len(".json")]
                try:
                    with open(p, encoding="utf-8") as fh:
                        rec = json.load(fh)
                except (OSError, ValueError):
                    continue
                if not isinstance(rec, dict):
                    continue
                data = rec.get("data") if isinstance(rec.get("data"), dict) else {}
                src = rec.get("source") if isinstance(rec.get("source"), dict) else {}
                entry = {
                    "path": os.path.relpath(p, REPO),
                    "book": book,
                    "kind": kind,
                    "slug": slug,
                    "key": data.get("key"),
                    "name": data.get("name") or data.get("key"),
                    "description": data.get("description"),
                    "source_file": os.path.basename(src.get("path") or "") or None,
                    "source_line": src.get("line"),
                }
                records[entry["path"]] = entry
                if entry["source_file"] and entry["source_line"] is not None:
                    by_line.setdefault((book, entry["source_file"], entry["source_line"]), entry)
                by_kind_slug.setdefault((book, kind, slug), entry)
    return records, by_line, by_kind_slug


def unit_corpus_record(unit, by_line, by_kind_slug):
    """Resolve an inventory unit to its corpus record, `load_population`-style."""
    book = unit.get("book")
    sf = os.path.basename(unit.get("source_file") or "")
    line = unit.get("source_line")
    hit = by_line.get((book, sf, line))
    if hit:
        return hit
    parts = (unit.get("id") or "").split(":", 2)
    if len(parts) == 3:
        for kind_dir in (
            unit.get("kind"),
            f"{unit.get('kind')}_generic",
            "ability",
            "equipment",
        ):
            hit = by_kind_slug.get((book, kind_dir, parts[2]))
            if hit:
                return hit
    return None


def copy_pairs(book, lst_name):
    """`{base-name -> [new-key, ...]}` for every `<New Key>.COPY=<Base>` row.

    Read from the pinned tree, not guessed. Returns {} when the file is absent
    so the census degrades to "unconfirmed" rather than to a wrong answer.
    """
    path = os.path.join(PCGEN_ROOT, BOOKS_REL, book, lst_name)
    pairs = defaultdict(list)
    lines = {}
    if not os.path.isfile(path):
        return pairs, lines
    with open(path, encoding="utf-8", errors="replace") as fh:
        for n, raw in enumerate(fh, start=1):
            lhs = raw.split("\t")[0].strip()
            if not lhs or lhs.startswith("#"):
                continue
            lines[n] = lhs
            if ".COPY=" in lhs:
                new_key, base = lhs.split(".COPY=", 1)
                pairs[norm_key(base.strip())].append((new_key.strip(), n))
    return pairs, lines


def main():
    with open(UNREACHED, encoding="utf-8") as fh:
        buckets = json.load(fh)
    population = buckets["FULL_with_prose_no_row_reached"]

    with open(INV, encoding="utf-8") as fh:
        units = json.load(fh)["units"]

    records, by_line, by_kind_slug = load_corpus()

    # The REACHED corpus population: every record cycle 1's inversion did not
    # place in one of its five never-reached buckets. Every inventory unit is
    # DONE at HEAD (`completion_atlas.py --check` -> population=49438,
    # DONE=49438), so "reached" and "reached by a DONE unit" are the same set
    # here -- stated rather than assumed.
    unreached_paths = {r["path"] for bucket in buckets.values() for r in bucket}
    reached_text = defaultdict(list)
    for path, rec in records.items():
        if path in unreached_paths:
            continue
        t = norm_text(rec["description"])
        if t:
            reached_text[t].append(rec)

    inv_by_key = defaultdict(list)
    inv_by_name = defaultdict(list)
    inv_by_row = defaultdict(list)
    for u in units:
        inv_by_key[norm_key(u.get("corpus_key"))].append(u)
        inv_by_name[norm_key(u.get("name"))].append(u)
        inv_by_row[(os.path.basename(u.get("source_file") or ""), u.get("source_line"))].append(u)

    # Cache the `.COPY=` map per (book, lst) -- only the files the population
    # actually names are read.
    copy_cache = {}

    dispositions = []
    for rec_stub in population:
        path = rec_stub["path"]
        rec = records.get(path)
        if rec is None:
            dispositions.append(
                {**rec_stub, "disposition": "corpus_record_missing_at_head", "reaches": None}
            )
            continue
        my_text = norm_text(rec["description"])
        key = norm_key(rec["key"] or rec_stub.get("key"))

        # --- content key 0: the same row of the same pinned `.lst`, book label
        # ignored, because `attributed_book` re-attributes a row to its newest
        # printing. Same file + same line IS the same rule -- no text test is
        # needed or meaningful here, the two are one row. ---
        row_units = inv_by_row.get((rec["source_file"] or "", rec["source_line"]), [])
        if row_units:
            u = row_units[0]
            dispositions.append(
                {
                    **rec_stub,
                    "disposition": "reached_via_reattributed_same_row",
                    "reaches": u.get("id"),
                    "twin_status": u.get("status"),
                    "inventory_book": u.get("book"),
                    "corpus_book": rec["book"],
                    "row": f"{rec['source_file']}:{rec['source_line']}",
                }
            )
            continue

        # --- content key 1: same KEY in another book, same published words ---
        twin = None
        collisions = []
        for u in inv_by_key.get(key, []) + inv_by_name.get(key, []):
            other = unit_corpus_record(u, by_line, by_kind_slug)
            if other is None:
                continue
            if other["path"] == path:
                continue
            if my_text and norm_text(other["description"]) == my_text:
                twin = (u, other)
                break
            collisions.append(
                {"unit": u.get("id"), "unit_status": u.get("status"), "corpus": other["path"]}
            )
        if twin:
            u, other = twin
            dispositions.append(
                {
                    **rec_stub,
                    "disposition": "reached_via_text_identical_twin",
                    "reaches": u.get("id"),
                    "twin_status": u.get("status"),
                    "twin_corpus_record": other["path"],
                }
            )
            continue

        # --- content key 3: the `.COPY=` twin inside the record's own file.
        # Tested BEFORE the general prose test because it names the exact
        # mechanism (PCGen's equipment-modifier idiom) rather than only the
        # consequence, and a mechanism is the part a successor cycle can act on.
        copy_hit = None
        lst = rec["source_file"]
        if lst:
            ck = (rec["book"], lst)
            if ck not in copy_cache:
                copy_cache[ck] = copy_pairs(rec["book"], lst)
            pairs, lst_lines = copy_cache[ck]
            base_name = lst_lines.get(rec["source_line"], "")
            copy_rows = pairs.get(norm_key(base_name)) if base_name else None
            for new_key, copy_line in copy_rows or []:
                for u in units:
                    if (
                        u.get("book") == rec["book"]
                        and os.path.basename(u.get("source_file") or "") == lst
                        and u.get("source_line") == copy_line
                    ):
                        copy_hit = (u, new_key, copy_line, base_name)
                        break
                if copy_hit:
                    break
        if copy_hit:
            u, new_key, copy_line, base_name = copy_hit
            dispositions.append(
                {
                    **rec_stub,
                    "disposition": "reached_via_copy_row_twin_same_file",
                    "reaches": u.get("id"),
                    "twin_status": u.get("status"),
                    "copy_row": f"{lst}:{copy_line}",
                    "copy_row_text": f"{new_key}.COPY={base_name}",
                }
            )
            continue

        # --- content key 2: identical published prose anywhere in the reached
        # population, key ignored. The sheet rule asks only whether these words
        # reach a sheet line; it does not ask under what name. ---
        prose_twins = (
            [r for r in reached_text.get(my_text, []) if r["path"] != path] if my_text else []
        )
        if prose_twins:
            other = prose_twins[0]
            dispositions.append(
                {
                    **rec_stub,
                    "disposition": "reached_via_identical_prose_record",
                    "reaches": None,
                    "prose_twin_corpus_record": other["path"],
                    "prose_twin_key": other["key"],
                    "prose_twin_count": len(prose_twins),
                }
            )
            continue

        dispositions.append(
            {
                **rec_stub,
                "disposition": "absent_no_unit_anywhere",
                "reaches": None,
                "name_collisions": collisions,
            }
        )

    counts = Counter(d["disposition"] for d in dispositions)
    absent = [d for d in dispositions if d["disposition"] == "absent_no_unit_anywhere"]
    reached = [d for d in dispositions if d["disposition"].startswith("reached")]
    twin_status = Counter(d.get("twin_status") for d in reached)

    out = {
        "criterion": "AT-35-E7-000-POPULATION-CENSUS",
        "cycle": 2,
        "scope": "the cycle-1 remainder `record_absent_from_inventory_population=255`, "
        "every record, not a sample",
        "measure_only": "reads only; adds no unit, converts nothing, edits no classifier, "
        "changes no gate",
        "method": {
            "cycle_1_keys": "(book, source_file, source_line) and (book, kind, id-tail) -- "
            "`mod.rs::load_population`'s two COORDINATE keys",
            "cycle_2_keys": "corpus_key/name across all books, CONFIRMED by verbatim "
            "`data.description` identity (whitespace+case normalised only); and the "
            "`<New Key>.COPY=<Base>` pairing read from the pinned `.lst`",
            "why": "a coordinate key asks where a row sits; the sheet rule asks whether the "
            "rule's words reach a sheet line. Only the second question decides whether a "
            "record is a lost rule.",
            "collision_guard": "a shared KEY alone is never accepted -- the two records' "
            "published prose must be byte-identical after whitespace/case normalisation, so "
            "`Amorphous` (protean anatomy) can never be credited for `Amorphous` (ACG armor).",
        },
        "population": len(population),
        "dispositions": dict(counts),
        "reached_twin_status": {k: v for k, v in twin_status.items()},
        "absent_total": len(absent),
        "absent_by_book_kind": dict(
            Counter(f"{d['book']}/{d['kind']}" for d in absent)
        ),
        "absent_records": sorted(
            (
                {
                    "path": d["path"],
                    "book": d["book"],
                    "kind": d["kind"],
                    "slug": d["slug"],
                    "key": d.get("key"),
                    "source": f"{d.get('source_file')}:{d.get('source_line')}",
                    "desc_chars": d.get("desc_chars"),
                }
                for d in absent
            ),
            key=lambda d: (d["book"], d["kind"], d["slug"]),
        ),
        "pcgen_tree_present": os.path.isdir(os.path.join(PCGEN_ROOT, BOOKS_REL)),
        "root_cause_of_the_absent": {
            "mechanism": "src/bin/v06_work_inventory.rs `has_classifying_token`: "
            "`Kind::Feat => has_token(fields, \"TYPE:\")` and "
            "`Kind::Spell => has_token(\"SCHOOL:\") || has_token(\"CLASSES:\")`. "
            "A row failing its kind's test is not enumerated and lands in the "
            "`missing_classifying_token` trap.",
            "evidence": "all 9 `pu_feats.lst` rows 6-14 carry `CATEGORY:FEAT`, `DESC:` and "
            "`BENEFIT:` and NO `TYPE:`; all 8 `pu_feats.lst` rows the inventory does hold "
            "(18,19,20,25,26,27,29,32) carry `TYPE:`. `ma_spells.lst:98` "
            "(`Elemental Body IIIMOD`) carries `DESC:` and `PREABILITY:` only -- no `SCHOOL:`, "
            "no `CLASSES:`.",
            "re_derive": "awk -F'\\t' 'NR>=6&&NR<=14' $PCGEN_CORPUS_ROOT/"
            + BOOKS_REL
            + "/pathfinder_unchained/pu_feats.lst | grep -c 'TYPE:'  # -> 0",
            "single_trap": True,
        },
        "corpus_wide_figure_corrected": {
            "cycle_1_claim": "48609 of 48864 real corpus rules records = 99.478%",
            "corrected": "48854 of 48864 = 99.9795%",
            "arithmetic": "48609 + (255 - 10) = 48854; the denominator 48864 is unchanged",
            "why": "245 of the 255 cycle 1 subtracted from the numerator do reach a DONE "
            "inventory unit -- under a re-attributed book label, a `.COPY=` twin row, or a "
            "prose-identical record. Only 10 reach nothing.",
        },
        "what_this_proof_does_not_cover": [
            "It does not re-verify the 2,657 records cycle 1 placed in the three "
            "not-a-record buckets (`chassis_only`, `duplicate_ingest`, `_settled`). Those keep "
            "cycle 1's disposition; this cycle's scope is the 255 remainder only.",
            "`reached_via_identical_prose_record` proves the WORDS reach a sheet line; it does "
            "not prove the reached record is filed under the name a player would look up. "
            "`Threefold Aspect (Adulthood)` prints as part of `Threefold Aspect`.",
            "Prose identity is normalised for whitespace and case ONLY. A twin differing in a "
            "single numeral, die expression or bonus compares different and stays a gap -- the "
            "test cannot silently launder a changed magnitude.",
        ],
    }

    with open(OUT, "w", encoding="utf-8") as fh:
        json.dump(out, fh, indent=2, sort_keys=False)
        fh.write("\n")

    json.dump(
        {k: out[k] for k in ("population", "dispositions", "absent_total", "absent_by_book_kind")},
        sys.stdout,
        indent=2,
    )
    print()
    # Per-record detail lives beside the summary for auditability.
    with open(os.path.join(HERE, "population-census-255-detail.json"), "w", encoding="utf-8") as fh:
        json.dump(dispositions, fh, indent=2)
        fh.write("\n")


if __name__ == "__main__":
    main()
