#!/usr/bin/env python3
"""SD-32 card 11, T9 — Product-Identity exposure audit (decisions.md §15).

**Read-only.** Transcribes nothing, ingests nothing, changes no corpus data.
Classifies every T9 unit (the full 2,712-unit population, not a sample)
against `docs/governance/ogl-pi-blacklist.md` and reports the real blocked
count, per kind and per book, naming the records -- so the operator can sign
off (or amend) the blacklist knowing the actual exposure.

Re-derive command (from repo root, worktree pinned to `decisions.md §13`'s
commit or later, oracle fetched to the repo-local slot):

    cargo build --locked --release --bin v06_work_inventory
    PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data \\
        <target>/release/v06_work_inventory --stdout-only > fresh_inventory.json
    python3 scripts/sd32_t9_pi_exposure_audit.py fresh_inventory.json \\
        --corpus-root <repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data

Method
------
For every T9 unit (the same six evidence-code families
`scripts/sd32_t9_census.py` filters to), this script:

1. Resolves the unit's `(source_file, source_line)` to a real file under
   `PCGEN_CORPUS_ROOT` (a basename index built once over the whole corpus
   tree -- `source_file` is a bare basename, not a path, per
   `transcribe_monster_tables.py::resolve_book_file`'s own documented
   finding that a book's own root is not always where its file lives).
2. Reads the WHOLE raw tab-separated row at that line -- not one field --
   because PI often lives in flavour text (`DESC:`), not in the field a
   evidence-code check happens to have looked at (`AGENTS.md`/dispatch
   brief method point 7).
3. Classifies it in three buckets, mirroring
   `src/rules_core/pi_screening.rs`'s actual production screen plus the
   blacklist's own §2.3 "per-record judgment" category, which the shipped
   screen (deliberately) treats as an automatic pass:

   - **blocked**: the row declares `NAMEISPI:YES` or `DESCISPI:YES`
     (`pi_screening::declared_product_identity`), OR any of the 57
     `PI_BLACKLIST_TERMS` (`pi_screening::PI_BLACKLIST_TERMS`) appears
     as a substring anywhere in the raw row.
   - **uncertain**: the row carries a free-text/prose tag
     (`DESC:`, `BENEFIT:`, `SPECIALS:`, `SA:` -- the tags
     `ogl-pi-blacklist.md §2.3` names as "requiring per-record judgment,
     not blanket-classifiable") with non-trivial content, is not
     `blocked` above, and so is exactly the case the blacklist's own
     DRAFT banner says to stop and ask about rather than guess clean.
     **This is the audit's central finding, not a defect in the
     method**: the production term-list scan silently treats every one
     of these as OGL today, and `decisions.md §15` exists because that
     silence is not proof of cleanliness (the Inner Sea Gods oracle-typo
     incident recorded in `ogl-pi-blacklist.md §4` is exactly this
     failure mode having already happened once).
   - **clear**: no PI declaration, no term-list hit, AND no free-text tag
     present at all -- a purely mechanical row (`ogl-pi-blacklist.md
     §2.2`, blanket OGL).

Limits, stated plainly (do not silently narrow `uncertain` to make the
report tidier):

- The 57-term list is a documented, bounded sample (20 deities + 34
  place/nation names + 3 per-book additions), not an exhaustive legal
  review -- `ogl-pi-blacklist.md`'s own DRAFT banner says so. A `clear`
  or `blocked` verdict from this script is only as good as that list.
- The free-text tag set (`DESC`, `BENEFIT`, `SPECIALS`, `SA`) is this
  script's own heuristic for "the row carries prose a human would need to
  read," not a field enumerated anywhere in the blacklist for every one
  of T9's six kinds -- `ogl-pi-blacklist.md §2.3` names it explicitly
  only for `SpellCacheData`/`EquipmentCacheData`/`FeatTableEntry`
  `description` and `RaceTraitEntry.detail`. Extending it to
  `companion`/`monster_ability`/`monster` rows generically is this
  script's own conservative choice, logged here so the operator can
  narrow it later if a per-kind review shows it is too wide.
"""
from __future__ import annotations

import argparse
import json
import os
import re
import sys
from collections import defaultdict

EVIDENCE_FAMILIES = {
    "spell": re.compile(r"^spell_key_absent_from_spell_list"),
    "companion": re.compile(r"^companion_absent_from_"),
    "feat": re.compile(r"^feat_key_absent_from_catalog"),
    "monster_ability": re.compile(r"^monster_ability_absent_from_"),
    "equipment": re.compile(r"^equipment_key_absent_from_equipment_tables"),
    "monster": re.compile(r"^monster_absent_from_"),
}

# `decisions.md §26`: imported from `pi_scrub.py`, the ONE shared home for
# this list (was byte-identical to `src/rules_core/pi_screening.rs::PI_BLACKLIST_TERMS`,
# 57 terms, plus 3 more added by `decisions.md §19a` amendment 3d -- see
# `pi_scrub.py`'s own docstring for the full provenance note, and
# `ogl-pi-blacklist.md §2.3c` for why the separate 61-term Rust production
# constant is not bumped here). Previously a second literal copy of this list
# lived in this file -- removed as part of `§26`'s duplication-drift fix.
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from pi_scrub import PI_BLACKLIST_TERMS  # noqa: E402

assert len(PI_BLACKLIST_TERMS) == 61, "term list drifted -- expected 57 + Aldori/Magaambya/Magaambyan + the ISG equipment lowercase-possessive addition (decisions.md §19a 3d, §12b)"

# `ogl-pi-blacklist.md §2.3`'s named per-record-judgment tags, widened (see
# module docstring "Limits") to every T9 kind's free-text-shaped PCGen tags.
FREE_TEXT_TAG_PREFIXES = ("DESC:", "BENEFIT:", "SPECIALS:", "SA:")

SOFT_HYPHEN = "­"


def load_units(path: str) -> list[dict]:
    data = json.load(open(path, encoding="utf-8"))
    return data["units"]


def t9_units(units: list[dict]) -> list[dict]:
    out = []
    for u in units:
        kind = u.get("kind")
        ev = u.get("evidence", "") or ""
        pat = EVIDENCE_FAMILIES.get(kind)
        if pat and pat.match(ev):
            out.append(u)
    return out


def build_basename_index(root: str) -> dict[str, list[str]]:
    """basename -> [full paths], over the whole corpus tree, built once."""
    index: dict[str, list[str]] = defaultdict(list)
    for dirpath, _dirnames, filenames in os.walk(root):
        for fn in filenames:
            if fn.endswith(".lst"):
                index[fn].append(os.path.join(dirpath, fn))
    return index


def resolve_source_file(index: dict[str, list[str]], book: str, source_file: str) -> tuple[str | None, str]:
    """Resolve a unit's bare `source_file` basename to one real path.

    Prefers a path whose directory components mention `book` (handles the
    common case cleanly); falls back to the sole match wherever it is
    (`inner_sea_gods`/`occult_adventures`-shaped units live under a
    `support/` subdirectory of a DIFFERENT book's own tree per
    `transcribe_monster_tables.py::resolve_book_file`'s documented finding).
    Returns `(path_or_None, reason)` -- `reason` is `"ok"`,
    `"not_found"`, or `"ambiguous:<n>"` when more than one candidate exists
    and none is uniquely book-scoped (the path is still returned as the
    first candidate in that case, flagged so the memo can list it).
    """
    candidates = index.get(source_file, [])
    if not candidates:
        return None, "not_found"
    if len(candidates) == 1:
        return candidates[0], "ok"
    book_norm = book.replace("_", "").lower()
    scoped = [c for c in candidates if book_norm in c.replace("_", "").replace("-", "").lower()]
    if len(scoped) == 1:
        return scoped[0], "ok"
    return candidates[0], f"ambiguous:{len(candidates)}"


def read_row(path: str, line_no: int) -> str:
    with open(path, encoding="utf-8", errors="replace") as handle:
        lines = handle.read().split("\n")
    if line_no < 1 or line_no > len(lines):
        return ""
    return lines[line_no - 1].replace(SOFT_HYPHEN, "-")


def token_value(row: str, prefix: str) -> str | None:
    for tok in row.split("\t"):
        tok = tok.strip()
        if tok.upper().startswith(prefix.upper()):
            return tok[len(prefix):].strip()
    return None


def classify_row(row: str) -> tuple[str, str]:
    """(bucket, reason) for one raw oracle row."""
    if not row.strip():
        return "uncertain", "source row empty/unreadable -- cannot classify, do not assume clear"

    nameispi = token_value(row, "NAMEISPI:")
    descispi = token_value(row, "DESCISPI:")
    if nameispi and nameispi.upper() == "YES":
        return "blocked", "NAMEISPI:YES (declared)"
    if descispi and descispi.upper() == "YES":
        return "blocked", "DESCISPI:YES (declared)"

    for term in PI_BLACKLIST_TERMS:
        if term in row:
            return "blocked", f'PI_BLACKLIST_TERMS hit: "{term}"'

    for tok in row.split("\t"):
        tok = tok.strip()
        for prefix in FREE_TEXT_TAG_PREFIXES:
            if tok.upper().startswith(prefix) and len(tok) > len(prefix) + 3:
                return "uncertain", f"free-text tag present ({prefix.rstrip(':')}), blacklist §2.3 requires per-record judgment"

    return "clear", "no PI declaration, no term-list hit, no free-text tag (blanket OGL, §2.2)"


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("inventory_json", help="fresh_inventory.json from v06_work_inventory --stdout-only")
    ap.add_argument("--corpus-root", required=True, help="PCGEN_CORPUS_ROOT (the .../pcgen/data dir)")
    ap.add_argument("--json-out", default=None, help="optional: write the full per-unit classification here")
    args = ap.parse_args()

    units = load_units(args.inventory_json)
    t9 = t9_units(units)

    index = build_basename_index(args.corpus_root)

    results = []
    unresolved = []
    for u in t9:
        path, reason = resolve_source_file(index, u["book"], u["source_file"])
        if path is None:
            unresolved.append(u)
            results.append({**u, "bucket": "uncertain", "classify_reason": f"source file not found under corpus root ({u['source_file']})"})
            continue
        row = read_row(path, u["source_line"])
        bucket, why = classify_row(row)
        results.append({**u, "bucket": bucket, "classify_reason": why, "resolved_path": path, "resolve_note": reason})

    by_kind = defaultdict(lambda: defaultdict(int))
    by_book = defaultdict(lambda: defaultdict(int))
    examples_blocked = defaultdict(list)

    for r in results:
        by_kind[r["kind"]][r["bucket"]] += 1
        by_book[r["book"]][r["bucket"]] += 1
        if r["bucket"] == "blocked" and len(examples_blocked[r["kind"]]) < 8:
            examples_blocked[r["kind"]].append(f"{r['book']}:{r['kind']}:{r['name']} ({r['classify_reason']})")

    print("=== T9 PI-exposure audit: totals ===")
    print(f"Total T9 units classified: {len(results)}")
    total_blocked = sum(v["blocked"] for v in by_kind.values())
    total_clear = sum(v["clear"] for v in by_kind.values())
    total_uncertain = sum(v["uncertain"] for v in by_kind.values())
    print(f"blocked={total_blocked} clear={total_clear} uncertain={total_uncertain}")
    print(f"unresolved source files (counted uncertain above): {len(unresolved)}")

    print()
    print("=== by kind ===")
    for k in EVIDENCE_FAMILIES:
        v = by_kind[k]
        total = v["blocked"] + v["clear"] + v["uncertain"]
        print(f"{k}\ttotal={total}\tblocked={v['blocked']}\tclear={v['clear']}\tuncertain={v['uncertain']}")

    print()
    print("=== by book ===")
    for book in sorted(by_book):
        v = by_book[book]
        total = v["blocked"] + v["clear"] + v["uncertain"]
        print(f"{book}\ttotal={total}\tblocked={v['blocked']}\tclear={v['clear']}\tuncertain={v['uncertain']}")

    print()
    print("=== fully-clear books (blocked=0 AND uncertain=0) ===")
    for book in sorted(by_book):
        v = by_book[book]
        if v["blocked"] == 0 and v["uncertain"] == 0:
            print(f"{book}\t{v['clear']}")

    print()
    print("=== example blocked records (up to 8 per kind) ===")
    for k, ex in examples_blocked.items():
        print(f"-- {k} --")
        for e in ex:
            print(f"  {e}")

    if unresolved:
        print()
        print(f"=== {len(unresolved)} units whose source_file could not be resolved under --corpus-root (treated as uncertain) ===")
        for u in unresolved[:20]:
            print(f"  {u['book']}:{u['kind']}:{u['name']} ({u['source_file']}:{u['source_line']})")
        if len(unresolved) > 20:
            print(f"  ... and {len(unresolved) - 20} more")

    if args.json_out:
        with open(args.json_out, "w", encoding="utf-8") as f:
            json.dump(results, f, indent=2)
        print(f"\nFull per-unit classification written to {args.json_out}")


if __name__ == "__main__":
    main()
