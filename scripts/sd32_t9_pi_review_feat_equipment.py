#!/usr/bin/env python3
"""SD-32 card 11, T9 -- per-record PI review for `feat` and `equipment` (decisions.md
§18, feat-equipment lane). **Read-only.** Transcribes nothing, ingests nothing,
changes no corpus data, does not amend `docs/governance/ogl-pi-blacklist.md`.

This script extends (does not redo) `scripts/sd32_t9_pi_exposure_audit.py`. It takes
that script's own `--json-out` classification and:

1. Re-derives the `feat`/`equipment`/`monster` subsets (this lane's scope plus the
   fully-resolved `monster` kind it re-checks per the dispatch brief).
2. Re-checks the `clear` bucket (feat + equipment + monster) with a normalized
   (case-folded + single-edit OCR-substitution) scan against the same 57-term
   `PI_BLACKLIST_TERMS` list -- the gap `ogl-pi-blacklist.md §4`'s Inner Sea Gods
   incident (`Cayden CaiLean`, `lrori`) recorded.
3. Traces `.COPY=`/`.MOD` rows in the `clear`/`uncertain` buckets back to their base
   item's own declared `NAMEISPI`/`DESCISPI` status -- the audit's own §8 gap 3,
   which this script answers for feat+equipment: **a `.COPY`/`.MOD` row inherits its
   base item's PI status.**

Re-derive command (from repo root, worktree pinned to decisions.md §18's commit or
later, oracle fetched to the repo-local slot):

    cargo build --locked --release --bin v06_work_inventory
    PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data \\
        <target>/release/v06_work_inventory --stdout-only > fresh_inventory.json
    python3 scripts/sd32_t9_pi_exposure_audit.py fresh_inventory.json \\
        --corpus-root <repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data \\
        --json-out t9_pi_classified.json
    python3 scripts/sd32_t9_pi_review_feat_equipment.py t9_pi_classified.json --corpus-root <same>

This is a REVIEW tool: it prints findings for a human/operator to act on. It does not
write license fields, does not mutate `t9_pi_classified.json`, and does not touch
`ogl-pi-blacklist.md`.
"""
from __future__ import annotations

import argparse
import json
import os
from collections import defaultdict

PI_BLACKLIST_TERMS = [
    "Iomedae", "Sarenrae", "Asmodeus", "Cayden Cailean", "Abadar", "Calistria", "Desna", "Erastil", "Gorum", "Gozreh",
    "Irori", "Lamashtu", "Nethys", "Norgorber", "Pharasma", "Rovagug", "Shelyn", "Torag", "Urgathoa", "Zon-Kuthon",
    "Golarion", "Absalom", "Cheliax", "Varisia", "Andoran", "Taldor", "Osirion", "Katapesh", "Ustalav", "Numeria",
    "Mwangi", "Tian Xia", "Avistan", "Garund", "Sarkoris", "Worldwound", "Vudra", "Kyonin", "Molthune", "Nidal",
    "Nirmathas", "Qadira", "Razmiran", "Rahadoum", "Galt", "Isger", "Lastwall", "Brevoy", "Druma", "Irrisen",
    "Jalmeray", "Thuvia", "Geb", "Nex",
    "Jarn",
    "Cayden CaiLean",
    "lrori",
]
assert len(PI_BLACKLIST_TERMS) == 57

# Golarion-specific proper nouns this lane found cited (in PREABILITY/prerequisite
# fields) that are NOT in PI_BLACKLIST_TERMS -- proposed additions, not applied here.
PROPOSED_TERM_ADDITIONS = ["Aldori", "Magaambya", "Magaambyan"]

SOFT_HYPHEN = "­"


def read_row(path: str, line_no: int) -> str:
    with open(path, encoding="utf-8", errors="replace") as h:
        lines = h.read().split("\n")
    if line_no < 1 or line_no > len(lines):
        return ""
    return lines[line_no - 1].replace(SOFT_HYPHEN, "-")


def token_value(row: str, prefix: str) -> str | None:
    for tok in row.split("\t"):
        tok = tok.strip()
        if tok.upper().startswith(prefix.upper()):
            return tok[len(prefix):].strip()
    return None


def casefold_hit(row: str, terms: list[str]) -> list[str]:
    rowcf = row.casefold()
    return [t for t in terms if t.casefold() in rowcf]


def ocr_variants(term: str) -> set[str]:
    variants = set()
    for i, ch in enumerate(term):
        if ch == "l":
            variants.add(term[:i] + "I" + term[i + 1:])
        if ch == "I":
            variants.add(term[:i] + "l" + term[i + 1:])
    if "rn" in term:
        variants.add(term.replace("rn", "m"))
    if "m" in term:
        variants.add(term.replace("m", "rn"))
    variants.discard(term)
    return variants


def ocr_hit(row: str, terms: list[str]) -> list[tuple[str, str]]:
    out = []
    for t in terms:
        for v in ocr_variants(t):
            if v in row:
                out.append((t, v))
    return out


def find_base_item_pi(corpus_root: str, book_dir_hint: str, base_name: str, index_cache: dict) -> str | None:
    """Best-effort: search the same resolved file's siblings for a bare (non-.COPY,
    non-.MOD) row whose leading key equals `base_name` and report its declared PI
    status, if any. Returns 'NAMEISPI:YES' / 'DESCISPI:YES' / None."""
    if base_name not in index_cache:
        return None
    return index_cache[base_name]


def build_key_pi_index(path: str) -> dict[str, str]:
    """One .lst file -> {bare KEY name: 'NAMEISPI:YES'|'DESCISPI:YES'} for every
    non-.MOD/.COPY row that declares PI. Used to resolve `.COPY=X` / `X.MOD` base
    references within the same file."""
    idx: dict[str, str] = {}
    try:
        with open(path, encoding="utf-8", errors="replace") as h:
            lines = h.read().split("\n")
    except OSError:
        return idx
    for line in lines:
        line = line.replace(SOFT_HYPHEN, "-")
        if not line.strip() or line.startswith("#"):
            continue
        first = line.split("\t")[0].strip()
        base = first.split(".COPY=")[0].split(".MOD")[0].strip()
        if not base or base == first and (".COPY=" in first or ".MOD" in first):
            pass
        nameispi = token_value(line, "NAMEISPI:")
        descispi = token_value(line, "DESCISPI:")
        declared = None
        if nameispi and nameispi.upper() == "YES":
            declared = "NAMEISPI:YES"
        elif descispi and descispi.upper() == "YES":
            declared = "DESCISPI:YES"
        if declared and ".COPY=" not in first and ".MOD" not in first:
            idx[base] = declared
    return idx


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("classified_json", help="t9_pi_classified.json from sd32_t9_pi_exposure_audit.py --json-out")
    ap.add_argument("--corpus-root", required=True)
    args = ap.parse_args()

    recs = json.load(open(args.classified_json, encoding="utf-8"))
    by_kb = defaultdict(lambda: defaultdict(list))
    for r in recs:
        by_kb[r["kind"]][r["bucket"]].append(r)

    print("=== feat/equipment/monster bucket sizes (re-derived from classified_json) ===")
    for k in ("feat", "equipment", "monster"):
        for b in ("blocked", "clear", "uncertain"):
            print(f"{k}/{b}: {len(by_kb[k][b])}")

    # --- 1. normalized re-check of clear buckets ---
    print()
    print("=== normalized (case-fold + OCR 1-edit) re-check of clear buckets ===")
    total_newly_blocked = 0
    total_newly_uncertain = 0
    for k in ("feat", "equipment", "monster"):
        clear = by_kb[k]["clear"]
        nb, nu = [], []
        for r in clear:
            row = read_row(r["resolved_path"], r["source_line"])
            cf = casefold_hit(row, PI_BLACKLIST_TERMS)
            if cf:
                nb.append((r["book"], r["name"], cf))
                continue
            ocr = ocr_hit(row, PI_BLACKLIST_TERMS)
            if ocr:
                nu.append((r["book"], r["name"], ocr))
        print(f"{k}: rechecked={len(clear)} newly_blocked={len(nb)} newly_uncertain={len(nu)}")
        for row in nb:
            print("   BLOCKED:", row)
        for row in nu:
            print("   UNCERTAIN:", row)
        total_newly_blocked += len(nb)
        total_newly_uncertain += len(nu)
    print(f"TOTAL newly_blocked={total_newly_blocked} newly_uncertain={total_newly_uncertain}")

    # --- 2. .COPY/.MOD base-item PI inheritance trace, feat+equipment clear+uncertain ---
    print()
    print("=== .COPY=/.MOD base-item PI inheritance trace (feat + equipment, clear+uncertain) ===")
    file_index_cache: dict[str, dict[str, str]] = {}
    inherited_blocked = []
    for k in ("feat", "equipment"):
        for b in ("clear", "uncertain"):
            for r in by_kb[k][b]:
                row = read_row(r["resolved_path"], r["source_line"])
                first = row.split("\t")[0].strip()
                if ".COPY=" in first:
                    base = first.split(".COPY=")[0].strip()
                elif ".MOD" in first:
                    base = first.split(".MOD")[0].strip()
                else:
                    continue
                path = r["resolved_path"]
                if path not in file_index_cache:
                    file_index_cache[path] = build_key_pi_index(path)
                declared = find_base_item_pi(args.corpus_root, r["book"], base, file_index_cache[path])
                if declared:
                    inherited_blocked.append((r["book"], k, r["name"], f"base '{base}' declares {declared} -> inherits blocked"))
    print(f"{len(inherited_blocked)} row(s) whose .COPY=/.MOD base declares NAMEISPI/DESCISPI (currently bucketed {['clear','uncertain']} by the base script, not tracing this):")
    for row in inherited_blocked:
        print("  ", row)

    print()
    print("=== proposed term-list additions found by this lane (not applied) ===")
    for t in PROPOSED_TERM_ADDITIONS:
        print(f"  {t}")


if __name__ == "__main__":
    main()
