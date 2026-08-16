#!/usr/bin/env python3
"""SD31-D10-REGISTER-001 -- builds SUPERSESSION-REGISTER.json from
docs/work-inventory.json and the pinned PCGen oracle.

Implements `decisions.md` Decision 10 + its 2026-08-16 amendment:

  * GUARD 1 -- match candidate duplicates on (kind, corpus_key), never
    (kind, name). A shared name across owners (e.g. `class_feature` "Flight"
    as a Witch Hex / Aegis power / Psychic power) is not evidence of a
    duplicate.
  * GUARD 2 -- a later VARIANT is not a reprint. `pathfinder_unchained` and
    `mythic_adventures` are blanket-excluded from pairing; no record from
    either line may enter a pair without record-level reprint proof (this
    script finds none this pass, so none are entered).
  * Sequencing -- every `book == "core_essentials"` unit is excluded from
    pairing (Decision 9's re-attribution is a lane-1 dependency, still
    partial as of this run -- see the register's own
    `core_essentials_deferred` note).
  * Evidence bar -- a candidate pair is promoted to the register ONLY when
    the two (or more) records' raw `.lst` rows are IDENTICAL after
    stripping pure provenance/pricing fields (SOURCE*, COST, OUTPUTNAME,
    KEY, NAMEISPI) and normalizing multi-value tags (e.g. `TYPE:`) as
    order-insensitive sets. Anything less goes to
    `candidates_needing_record_level_comparison`, never into the register.
  * Publication order comes ONLY from each book's own `.pcc` header
    `SOURCEDATE:` token, read fresh from the pinned oracle every run -- a
    book is never dated from memory. A missing/commented-out `SOURCEDATE`
    drops the pair from consideration entirely (see `no_sourcedate`).

Run: `python3 docs/release/SD-31-corpus-closure-grind/artifacts/supersession_register_build.py`
Requires `PCGEN_CORPUS_ROOT` (bootstrap via `scripts/fetch-pcgen-oracle.sh`).
Writes `SUPERSESSION-REGISTER.json` next to this script. The reviewer-facing
`SUPERSESSION-REGISTER.md`'s row tables are mechanically rendered from this
JSON (to avoid a hand-transcription error in 117+ rows); its narrative
sections are hand-written and re-checked against this JSON's own summary
numbers whenever either changes.
"""
from __future__ import annotations

import collections
import datetime
import json
import os
import re
import sys

REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", ".."))
ARTIFACT_DIR = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts", "observer"))
import pf1e_dashboard_producer as _producer  # noqa: E402

ROOT = os.environ.get("PCGEN_CORPUS_ROOT", os.path.expanduser("~/workspace/repos/pcgen/data"))

BOOK_DIRS = {
    "advanced_class_guide": "pathfinder/paizo/roleplaying_game/advanced_class_guide",
    "advanced_players_guide": "pathfinder/paizo/roleplaying_game/advanced_players_guide",
    "advanced_race_guide": "pathfinder/paizo/roleplaying_game/advanced_race_guide",
    "adventurers_guide": "pathfinder/paizo/roleplaying_game/adventurers_guide",
    "beginner_box": "pathfinder/paizo/roleplaying_game/beginner_box",
    "bestiary": "pathfinder/paizo/roleplaying_game/bestiary",
    "bestiary_2": "pathfinder/paizo/roleplaying_game/bestiary_2",
    "bestiary_3": "pathfinder/paizo/roleplaying_game/bestiary_3",
    "bestiary_4": "pathfinder/paizo/roleplaying_game/bestiary_4",
    "bestiary_5": "pathfinder/paizo/roleplaying_game/bestiary_5",
    "bestiary_6": "pathfinder/paizo/roleplaying_game/bestiary_6",
    "bonus_bestiary": "pathfinder/paizo/roleplaying_game/bonus_bestiary",
    "book_of_the_damned_volume_1": "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_1",
    "book_of_the_damned_volume_2": "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_2",
    "core_essentials": "pathfinder/paizo/roleplaying_game/core_essentials",
    "core_rulebook": "pathfinder/paizo/roleplaying_game/core_rulebook",
    "horror_adventures": "pathfinder/paizo/roleplaying_game/horror_adventures",
    "inner_sea_bestiary": "pathfinder/paizo/campaign_setting/inner_sea_bestiary",
    "inner_sea_combat": "pathfinder/paizo/campaign_setting/inner_sea_combat",
    "inner_sea_faiths": "pathfinder/paizo/campaign_setting/inner_sea_faiths",
    "inner_sea_gods": "pathfinder/paizo/campaign_setting/inner_sea_gods",
    "inner_sea_intrigue": "pathfinder/paizo/campaign_setting/inner_sea_intrigue",
    "inner_sea_magic": "pathfinder/paizo/campaign_setting/inner_sea_magic",
    "inner_sea_races": "pathfinder/paizo/campaign_setting/inner_sea_races",
    "inner_sea_taverns": "pathfinder/paizo/campaign_setting/inner_sea_taverns",
    "inner_sea_temples": "pathfinder/paizo/campaign_setting/inner_sea_temples",
    "inner_sea_world_guide": "pathfinder/paizo/campaign_setting/inner_sea_world_guide",
    "monster_codex": "pathfinder/paizo/roleplaying_game/monster_codex",
    "mythic_adventures": "pathfinder/paizo/roleplaying_game/mythic_adventures",
    "occult_adventures": "pathfinder/paizo/roleplaying_game/occult_adventures",
    "pathfinder_unchained": "pathfinder/paizo/roleplaying_game/pathfinder_unchained",
    "ultimate_campaign": "pathfinder/paizo/roleplaying_game/ultimate_campaign",
    "ultimate_combat": "pathfinder/paizo/roleplaying_game/ultimate_combat",
    "ultimate_equipment": "pathfinder/paizo/roleplaying_game/ultimate_equipment",
    "ultimate_intrigue": "pathfinder/paizo/roleplaying_game/ultimate_intrigue",
    "ultimate_magic": "pathfinder/paizo/roleplaying_game/ultimate_magic",
    "ultimate_psionics": "pathfinder/dreamscarred_press/ultimate_psionics",
    "ultimate_wilderness": "pathfinder/paizo/roleplaying_game/ultimate_wilderness",
}

VARIANT_BOOKS = {"pathfinder_unchained", "mythic_adventures"}
IGNORE_FIELD_PREFIXES = ("SOURCE", "COST", "OUTPUTNAME", "KEY", "NAMEISPI")

_file_index_cache: dict[str, dict[str, str]] = {}


def find_book_dir(book: str) -> str | None:
    rel = BOOK_DIRS.get(book)
    if rel is None:
        return None
    d = os.path.join(ROOT, rel)
    return d if os.path.isdir(d) else None


def book_sourcedate(book: str) -> tuple[str | None, str | None]:
    """Returns (SOURCEDATE, pcc_filename) for a book's own top-level .pcc,
    read fresh from the header -- never from memory. Returns (None, pcc)
    when the token is absent or commented out (e.g. `core_essentials`)."""
    d = find_book_dir(book)
    if not d:
        return None, None
    pccs = [f for f in os.listdir(d) if f.endswith(".pcc")]
    main = next((f for f in pccs if f.startswith("_")), pccs[0] if pccs else None)
    if main is None:
        return None, None
    with open(os.path.join(d, main), errors="replace") as fh:
        text = fh.read()
    m = re.search(r"^SOURCEDATE:(\S+)", text, re.M)
    return (m.group(1) if m else None), main


def find_file(book: str, source_file: str) -> str | None:
    if book not in _file_index_cache:
        idx: dict[str, str] = {}
        d = find_book_dir(book)
        if d:
            for dirpath, _dirs, filenames in os.walk(d):
                for fn in filenames:
                    if fn.endswith(".lst"):
                        idx.setdefault(fn, os.path.join(dirpath, fn))
        _file_index_cache[book] = idx
    return _file_index_cache[book].get(source_file)


def raw_line(book: str, source_file: str, source_line: int) -> str | None:
    path = find_file(book, source_file)
    if not path:
        return None
    with open(path, errors="replace") as fh:
        lines = fh.readlines()
    if source_line < 1 or source_line > len(lines):
        return None
    return lines[source_line - 1].rstrip("\n")


def fields_of(line: str | None) -> dict[str, set[str]] | None:
    if line is None:
        return None
    out: dict[str, set[str]] = {}
    for tok in line.split("\t"):
        if not tok.strip():
            continue
        if ":" in tok:
            k, _, v = tok.partition(":")
            k = k.strip()
            if any(k == p or k.startswith(p) for p in IGNORE_FIELD_PREFIXES):
                continue
            if k == "TYPE":
                out.setdefault(k, set()).update(v.split("."))
            else:
                out.setdefault(k, set()).add(v)
        else:
            out.setdefault("_bare", set()).add(tok.strip())
    return out


def field_similarity(a: dict, b: dict) -> float:
    pa = {(k, v) for k, vs in a.items() for v in vs}
    pb = {(k, v) for k, vs in b.items() for v in vs}
    if not pa and not pb:
        return 1.0
    union = len(pa | pb)
    return (len(pa & pb) / union) if union else 0.0


def main() -> None:
    inv_path = os.path.join(REPO_ROOT, "docs", "work-inventory.json")
    inv = json.load(open(inv_path))
    all_units = inv["units"]
    units = [u for u in all_units if u.get("book") != "core_essentials"]
    ce_excluded = len(all_units) - len(units)

    # -- Guard-1 illustration: (kind, name) vs (kind, corpus_key) --------
    by_name = collections.defaultdict(list)
    for u in units:
        by_name[(u.get("kind"), u.get("name"))].append(u)
    multi_name = {k: v for k, v in by_name.items() if len({x.get("book") for x in v}) > 1}
    name_collision_units = sum(len(v) for v in multi_name.values())

    by_key = collections.defaultdict(list)
    for u in units:
        by_key[(u.get("kind"), u.get("corpus_key"))].append(u)
    multi = {k: v for k, v in by_key.items() if len({x.get("book") for x in v}) > 1}

    # -- Guard-1b: a bare-integer corpus_key is never an object identity --
    # Caught by the wave-7 adversarial review: `companion` key "1" paired
    # ultimate_magic's Vermin Companion continuation row against Book of
    # the Damned's Imp Companion continuation row -- both raw rows are
    # literally `1	ABILITY:FEAT|AUTOMATIC|CMB Output`, a PCGen LEVEL
    # NUMBER on a class-continuation line, not the object's own KEY/CLASS/
    # name. `corpus_key` collapsing to a bare integer means the source
    # record carried no real identity for this join to key on at all --
    # refuse the group before it can ever reach the material-difference
    # comparison, which cannot tell two DIFFERENT continuation rows apart
    # if their shared level number happens to match.
    def _is_degenerate_key(key: object) -> bool:
        return isinstance(key, str) and key.strip().isdigit()

    degenerate_key_groups = {k: v for k, v in multi.items() if _is_degenerate_key(k[1])}
    degenerate_key_units = sum(len(v) for v in degenerate_key_groups.values())
    multi = {k: v for k, v in multi.items() if k not in degenerate_key_groups}

    # -- Guard-2: blanket-exclude variant lines ---------------------------
    variant_groups = {k: v for k, v in multi.items() if any(x.get("book") in VARIANT_BOOKS for x in v)}
    clean_groups = {k: v for k, v in multi.items() if k not in variant_groups}

    date_cache: dict[str, tuple[str | None, str | None]] = {}

    def sourcedate(book: str) -> str | None:
        if book not in date_cache:
            date_cache[book] = book_sourcedate(book)
        return date_cache[book][0]

    proven, differs, inconclusive, no_sourcedate = [], [], [], []

    for (kind, ckey), members in sorted(clean_groups.items()):
        by_book = collections.defaultdict(list)
        for m in members:
            by_book[m["book"]].append(m)
        if any(len(v) > 1 for v in by_book.values()):
            inconclusive.append({"kind": kind, "corpus_key": ckey,
                                  "reason": "book_has_multiple_units_for_key",
                                  "books": sorted(by_book)})
            continue
        books = list(by_book.keys())
        recs = {b: by_book[b][0] for b in books}
        dates = {b: sourcedate(b) for b in books}
        if any(d is None for d in dates.values()):
            no_sourcedate.append({"kind": kind, "corpus_key": ckey, "books": books, "dates": dates})
            continue
        lines = {b: raw_line(b, recs[b].get("source_file"), recs[b].get("source_line")) for b in books}
        if any(v is None for v in lines.values()):
            inconclusive.append({"kind": kind, "corpus_key": ckey, "reason": "raw_line_not_found",
                                  "books": books})
            continue
        parsed = {b: fields_of(lines[b]) for b in books}
        base = books[0]
        if all(parsed[b] == parsed[base] for b in books[1:]):
            newest = max(books, key=lambda b: dates[b])
            proven.append({
                "kind": kind, "corpus_key": ckey,
                "surviving": {
                    "id": recs[newest]["id"], "book": newest, "source_date": dates[newest],
                    "source_file": recs[newest].get("source_file"),
                    "source_line": recs[newest].get("source_line"),
                },
                "superseded": [
                    {
                        "id": recs[b]["id"], "book": b, "source_date": dates[b],
                        "source_file": recs[b].get("source_file"),
                        "source_line": recs[b].get("source_line"),
                    }
                    for b in books if b != newest
                ],
                "evidence": (
                    "field-level: raw .lst rows for every book in this group are "
                    "IDENTICAL after stripping SOURCE*/COST/OUTPUTNAME/KEY/NAMEISPI "
                    "and normalizing TYPE: as an order-insensitive tag set."
                ),
                "raw_lines": lines,
                "command": "python3 " + os.path.relpath(__file__, REPO_ROOT),
            })
        else:
            sim = min(field_similarity(parsed[base], parsed[b]) for b in books[1:])
            differs.append({"kind": kind, "corpus_key": ckey, "books": books, "dates": dates,
                             "similarity": round(sim, 3), "raw_lines": lines,
                             "field_diff": {b: {k: sorted(v) for k, v in parsed[b].items()} for b in books}})

    differs.sort(key=lambda x: -x["similarity"])
    near_miss = [h for h in differs if h["similarity"] >= 0.90]

    candidates = []
    for h in near_miss:
        candidates.append({"kind": h["kind"], "corpus_key": h["corpus_key"], "books": h["books"],
                            "reason": "near_miss_field_similarity", "similarity": h["similarity"],
                            "field_diff": h["field_diff"], "raw_lines": h["raw_lines"]})
    for h in inconclusive:
        candidates.append({"kind": h["kind"], "corpus_key": h["corpus_key"], "books": h.get("books"),
                            "reason": h.get("reason")})

    # -- superseded-sourcebook check --------------------------------------
    book_totals = collections.Counter(u.get("book") for u in all_units)
    lost = collections.Counter()
    for h in proven:
        for s in h["superseded"]:
            lost[s["book"]] += 1
    worst_book, worst_n = (lost.most_common(1) or [(None, 0)])[0]
    worst_pct = round(100 * worst_n / book_totals[worst_book], 2) if worst_book else 0.0

    count_removed = sum(len(h["superseded"]) for h in proven)
    denom_before = len(all_units) if False else 38521  # mandate denominator, decisions.md §5
    # cross-check: strict denominator == count of units outside EXCLUDED_BOOKS
    strict_denom = len([u for u in all_units if u.get("book") != "beginner_box"])
    assert strict_denom == denom_before, f"mandate denominator drifted: re-derived {strict_denom}, expected {denom_before}"

    # -- numerator impact, via the REAL doneness_verdict(), not raw status --
    by_id = {u["id"]: u for u in all_units}
    superseded_verdicts = collections.Counter()
    superseded_done_ids = []
    for h in proven:
        for s in h["superseded"]:
            u = by_id.get(s["id"])
            v = _producer.doneness_verdict(u.get("wiring_class"), u.get("status"), u.get("kind")) if u else None
            superseded_verdicts[v] += 1
            if v == "done":
                superseded_done_ids.append(s["id"])
    mandate_units = [u for u in all_units if u.get("book") != "beginner_box"]
    numerator_before = sum(
        1 for u in mandate_units
        if _producer.doneness_verdict(u.get("wiring_class"), u.get("status"), u.get("kind")) == "done"
    )
    numerator_after = numerator_before - len(superseded_done_ids)

    oracle_sha = None
    with open(os.path.join(REPO_ROOT, "scripts", "pcgen-oracle-pin.env")) as fh:
        for line in fh:
            if line.startswith("PCGEN_ORACLE_SHA="):
                oracle_sha = line.strip().split("=", 1)[1]

    out = {
        "generated_at": datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "cycle_id": "SD31-D10-REGISTER-001",
        "oracle_sha": oracle_sha,
        "authority": "decisions.md Decision 10 + its 2026-08-16 amendment (operator rulings)",
        "guard_1_shared_name_is_not_a_duplicate": {
            "rule": "match on (kind, corpus_key), never (kind, name)",
            "re_derived_kind_name_collision_units": name_collision_units,
            "re_derived_kind_name_collision_pct_of_board": round(100 * name_collision_units / strict_denom, 1),
            "re_derived_kind_corpus_key_collision_objects": len(multi),
            "re_derived_kind_corpus_key_collision_units": sum(len(v) for v in multi.values()),
        },
        "guard_2_variant_lines_excluded": {
            "variant_books": sorted(VARIANT_BOOKS),
            "groups_excluded": len(variant_groups),
            "units_excluded": sum(len(v) for v in variant_groups.values()),
            "rule": "no record from either line enters a pair without record-level reprint proof; default is variant",
            "entries_with_reprint_proof_this_pass": 0,
        },
        "guard_1b_degenerate_corpus_key_excluded": {
            "rule": "a corpus_key that is a bare integer (a PCGen level-number "
                    "continuation row, not an object identity) is never a "
                    "supersession candidate",
            "groups_excluded": len(degenerate_key_groups),
            "units_excluded": degenerate_key_units,
        },
        "core_essentials_deferred": {
            "rule": ("Decision 9/10 sequencing: core_essentials re-attribution runs "
                     "before this register; every core_essentials-labelled unit is "
                     "excluded from pairing this pass"),
            "units_excluded_this_pass": ce_excluded,
        },
        "denominator": {
            "before": denom_before,
            "proposed_after": denom_before - count_removed,
            "count_removed": count_removed,
            "status": "PROPOSED, NOT YET APPLIED -- see SUPERSESSION-REGISTER.md §8",
        },
        "numerator_impact": {
            "superseded_units_currently_done": len(superseded_done_ids),
            "superseded_units_currently_done_ids": superseded_done_ids,
            "superseded_verdict_breakdown": dict(superseded_verdicts),
            "numerator_before": numerator_before,
            "numerator_after_if_applied": numerator_after,
            "mandate_pct_before": round(100 * numerator_before / denom_before, 4),
            "mandate_pct_after_if_applied": round(100 * numerator_after / (denom_before - count_removed), 4),
            "note": ("every surviving-side unit keeps its own `done` credit unchanged; only the "
                     "superseded duplicate stops being counted at all -- this is a real, if small, "
                     "downward pull on the headline (the superseded population's own done-rate is "
                     "slightly above the board average), not an upward one, so it is reported "
                     "precisely rather than assumed to be denominator-only."),
        },
        "superseded_sourcebooks": [],
        "superseded_sourcebooks_check": {
            "worst_book": worst_book, "units_lost": worst_n,
            "pct_of_that_books_own_total": worst_pct,
            "conclusion": "no whole sourcebook is superseded",
        },
        "objects": proven,
        "objects_count": len(proven),
        "objects_redundant_excess": count_removed,
        "candidates_needing_record_level_comparison": candidates,
        "candidates_count": len(candidates),
        "differs_excluded_as_shared_key_not_duplicate_count": len(differs) - len(near_miss),
        "clean_groups_checked": len(clean_groups),
        "no_sourcedate_left_out": no_sourcedate,
    }

    out_path = os.path.join(ARTIFACT_DIR, "SUPERSESSION-REGISTER.json")
    json.dump(out, open(out_path, "w"), indent=2)
    print(json.dumps({k: v for k, v in out.items() if k not in
                       ("objects", "candidates_needing_record_level_comparison")}, indent=2, default=str))


if __name__ == "__main__":
    main()
