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
import re
import sys
from collections import defaultdict

# `decisions.md §26`: `PI_BLACKLIST_TERMS`, `canonicalize`, and
# `normalized_term_hit` moved to the shared `pi_scrub.py` (this file's own
# copies drifted from `sd32_t9_pi_review_companion_monsterability.py`'s and
# `sd32_t9_pi_review_spell.py`'s independent copies -- the exact duplication
# shape `decisions.md §17` names). Re-exported here unchanged (by name) so
# every existing importer of this module (`ingest_ability.py`,
# `ingest_class.py`, `ingest_generic_kind.py`, `ingest_race_trait_generic.py`,
# `ingest_simple_filename_kinds.py`, `regen_all_renamed_pi_scrub.py`,
# `pi_key_rawtokens_audit.py`, `sd32_t9_pi_final_disposition.py`, `pi_scrub.py`
# itself) keeps working with no changes.
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from pi_scrub import (  # noqa: E402
    PI_BLACKLIST_TERMS,
    SOFT_HYPHEN,
    canonicalize,
    normalized_term_hit,
)

assert len(PI_BLACKLIST_TERMS) == 61, "term list drifted -- expected 57 + Aldori/Magaambya/Magaambyan + the ISG equipment lowercase-possessive addition (decisions.md §19a 3d, §12b)"


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


FREE_TEXT_TAG_PREFIXES = ("DESC:", "BENEFIT:", "SPECIALS:", "SA:")


def extract_free_text(row: str) -> str:
    """Scope the normalized scan to prose tags only, not the whole raw row.

    The whole row also carries `KEY:`/`DEFINE:`/`BONUS:` camelCase variable-name
    soup (e.g. `...damageBonus`) that, once case-folded, produces false hits for
    short blacklist terms (`Geb` inside `damageBonus`) -- the same false-positive
    class `sd32_t9_pi_review_companion_monsterability.py::normalized_scan`
    documents and fixes the same way. PI (OGL §1(e)) lives in flavour prose, not
    in a schema's own internal variable names.
    """
    parts = []
    for tok in row.split("\t"):
        tok = tok.strip()
        for prefix in FREE_TEXT_TAG_PREFIXES:
            if tok.upper().startswith(prefix):
                # `|` is PCGen's own field/sub-value delimiter, not prose -- stop
                # at the first one so a `DESC:text|Var1|Var2` row's trailing
                # substitution-variable names don't get scanned as if they were
                # sentence content either.
                parts.append(tok[len(prefix):].split("|", 1)[0])
    return " ".join(parts).strip()


# `canonicalize`/`normalized_term_hit` (decisions.md §19a amendment 3b's
# case-fold + bounded-OCR-confusion + word-boundary scan, plus `§26`'s
# "Jarn"/"jam" collision guard) now live in `pi_scrub.py` and are imported
# above -- this file no longer defines its own copy.


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

    # --- 1. normalized (decisions.md §19a amendment 3b) re-check of clear buckets ---
    print()
    print("=== normalized (case-fold + bounded OCR-confusion, word-boundary) re-check of clear buckets ===")
    total_newly_blocked = 0
    for k in ("feat", "equipment", "monster"):
        clear = by_kb[k]["clear"]
        nb = []
        for r in clear:
            row = read_row(r["resolved_path"], r["source_line"])
            hit = normalized_term_hit(extract_free_text(row))
            if hit:
                nb.append((r["book"], r["name"], hit))
        print(f"{k}: rechecked={len(clear)} newly_blocked={len(nb)}")
        for row in nb:
            print("   BLOCKED:", row)
        total_newly_blocked += len(nb)
    print(f"TOTAL newly_blocked={total_newly_blocked}")

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
    print("=== term-list additions applied this cycle (decisions.md §19a amendment 3d) ===")
    for t in ("Aldori", "Magaambya", "Magaambyan"):
        print(f"  {t}")


if __name__ == "__main__":
    main()
