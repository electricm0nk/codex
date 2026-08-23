#!/usr/bin/env python3
"""SD-32 card 11, T9 -- final PI disposition after operator sign-off
(`decisions.md §19`). **Read-only.** Transcribes nothing, ingests nothing,
changes no corpus data, does not touch `data/corpus/**`, and does not
mint kanban row 11 (`t9-pi-signoff-application` leaves it `in-progress` --
this is disposition evidence for a separate onboarding cycle, not
onboarding itself).

Aggregates the four kind-scoped review scripts (each already ported the
`decisions.md §19a` amendments 3b/3c, and `sd32_t9_pi_review_companion_
monsterability.py` additionally applies §19b/§19c) into one final
per-kind and per-book `blocked` / `clear` / `still_undecidable` table,
so the whole disposition has a single re-derive command instead of four
separately-read outputs.

Re-derive (from repo root, oracle bootstrapped to the repo-local slot):

    cargo build --locked --release --bin v06_work_inventory
    "$CARGO_TARGET_DIR/release/v06_work_inventory" --stdout-only \\
        > fresh_inventory.json
    python3 scripts/sd32_t9_pi_exposure_audit.py fresh_inventory.json \\
        --corpus-root "$PCGEN_CORPUS_ROOT" --json-out t9_pi_classified.json
    python3 scripts/sd32_t9_pi_final_disposition.py fresh_inventory.json \\
        t9_pi_classified.json --corpus-root "$PCGEN_CORPUS_ROOT"
"""
from __future__ import annotations

import argparse
import importlib.util
import json
import sys
from collections import defaultdict
from pathlib import Path

_HERE = Path(__file__).resolve().parent


def _load(name: str, filename: str):
    spec = importlib.util.spec_from_file_location(name, _HERE / filename)
    mod = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    spec.loader.exec_module(mod)
    return mod


audit = _load("sd32_t9_pi_exposure_audit", "sd32_t9_pi_exposure_audit.py")
fe = _load("sd32_t9_pi_review_feat_equipment", "sd32_t9_pi_review_feat_equipment.py")
cm = _load("sd32_t9_pi_review_companion_monsterability", "sd32_t9_pi_review_companion_monsterability.py")
spell_mod = _load("sd32_t9_pi_review_spell", "sd32_t9_pi_review_spell.py")


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("inventory_json")
    ap.add_argument("classified_json", help="t9_pi_classified.json from sd32_t9_pi_exposure_audit.py --json-out")
    ap.add_argument("--corpus-root", required=True)
    args = ap.parse_args()

    units = audit.load_units(args.inventory_json)
    t9 = audit.t9_units(units)
    index = audit.build_basename_index(args.corpus_root)

    final: list[dict] = []

    # --- spell: sd32_t9_pi_review_spell.py's own per-record review ---
    spells = [u for u in t9 if u["kind"] == "spell"]
    for u in spells:
        path, resolve_note = audit.resolve_source_file(index, u["book"], u["source_file"])
        row = audit.read_row(path, u["source_line"]) if path else ""
        bucket, _why = audit.classify_row(row) if path else ("still_undecidable", "source file not found")
        if bucket == "blocked":
            final.append({**u, "final_bucket": "blocked"})
            continue
        norm_hits = spell_mod.normalized_term_hits(row)
        if norm_hits:
            final.append({**u, "final_bucket": "blocked"})
            continue
        if bucket == "clear":
            final.append({**u, "final_bucket": "clear"})
            continue
        # uncertain -> per-record content read (spell_mod.unlisted_proper_noun_candidates)
        ft = spell_mod.extract_freetext(row)
        suspects = spell_mod.unlisted_proper_noun_candidates(ft)
        final.append({**u, "final_bucket": "still_undecidable" if suspects else "clear"})

    # --- feat + equipment + monster: sd32_t9_pi_review_feat_equipment.py's rules ---
    recs = json.load(open(args.classified_json, encoding="utf-8"))
    by_kb = defaultdict(lambda: defaultdict(list))
    for r in recs:
        by_kb[r["kind"]][r["bucket"]].append(r)

    # inheritance trace (3c), same as the script's own main()
    file_index_cache: dict[str, dict[str, str]] = {}
    inherited_blocked_names: set[tuple[str, str, str]] = set()
    for k in ("feat", "equipment"):
        for b in ("clear", "uncertain"):
            for r in by_kb[k][b]:
                row = fe.read_row(r["resolved_path"], r["source_line"])
                first = row.split("\t")[0].strip()
                if ".COPY=" in first:
                    base = first.split(".COPY=")[0].strip()
                elif ".MOD" in first:
                    base = first.split(".MOD")[0].strip()
                else:
                    continue
                path = r["resolved_path"]
                if path not in file_index_cache:
                    file_index_cache[path] = fe.build_key_pi_index(path)
                declared = fe.find_base_item_pi(args.corpus_root, r["book"], base, file_index_cache[path])
                if declared:
                    inherited_blocked_names.add((r["book"], k, r["name"]))

    # Mantis Blade: named individual case (§4.3), not resolved by §19a-c -- stays
    # still_undecidable explicitly (its base is clean; its OWN row carries
    # independent flavor text citing an OGL-published class name).
    NAMED_STILL_UNDECIDABLE = {("adventurers_guide", "equipment", "Mantis Blade")}

    for k in ("feat", "equipment", "monster"):
        for b in ("blocked", "clear", "uncertain"):
            for r in by_kb[k][b]:
                key = (r["book"], k, r["name"])
                if b == "blocked":
                    final.append({**r, "final_bucket": "blocked"})
                    continue
                if key in inherited_blocked_names:
                    final.append({**r, "final_bucket": "blocked"})
                    continue
                if key in NAMED_STILL_UNDECIDABLE:
                    final.append({**r, "final_bucket": "still_undecidable"})
                    continue
                row = fe.read_row(r["resolved_path"], r["source_line"])
                hit = fe.normalized_term_hit(fe.extract_free_text(row))
                if hit:
                    final.append({**r, "final_bucket": "blocked"})
                    continue
                if b == "clear":
                    final.append({**r, "final_bucket": "clear"})
                    continue
                # b == "uncertain": feat's 184 and equipment's 4 were read in
                # full by the committed review memo
                # (t9-pi-review-feat-equipment.md §2/§3) and found generic --
                # no automated content classifier exists for feat/equipment
                # prose (unlike spell/companion), so this script trusts that
                # already-committed, independently-reproduced human read
                # rather than re-deriving it: `clear`.
                final.append({**r, "final_bucket": "clear"})

    # --- companion + monster_ability: sd32_t9_pi_review_companion_monsterability.py ---
    for u in t9:
        if u["kind"] not in ("companion", "monster_ability"):
            continue
        path, reason = audit.resolve_source_file(index, u["book"], u["source_file"])
        row = audit.read_row(path, u["source_line"]) if path else ""
        exact_bucket, exact_reason = audit.classify_row(row) if path else ("uncertain", "not_found")
        free_text_for_scan = cm.extract_free_text(row)
        norm_hit = cm.normalized_scan(free_text_for_scan)
        if exact_bucket == "blocked" or norm_hit is not None:
            final.append({**u, "final_bucket": "blocked"})
        elif exact_bucket == "uncertain" and u["kind"] == "monster_ability":
            final.append({**u, "final_bucket": "clear"})  # §19b
        elif exact_bucket == "uncertain":
            content_bucket, _reason = cm.classify_uncertain_content(free_text_for_scan)
            final.append({**u, "final_bucket": content_bucket})
        else:
            final.append({**u, "final_bucket": "clear"})

    # --- aggregate ---
    by_kind = defaultdict(lambda: defaultdict(int))
    by_book = defaultdict(lambda: defaultdict(int))
    for r in final:
        by_kind[r["kind"]][r["final_bucket"]] += 1
        by_book[r["book"]][r["final_bucket"]] += 1

    print(f"=== T9 final disposition (post decisions.md §19a-c), total units: {len(final)} ===")
    print()
    print("=== by kind ===")
    for k in ("spell", "feat", "equipment", "monster", "companion", "monster_ability"):
        v = by_kind[k]
        total = v["blocked"] + v["clear"] + v["still_undecidable"]
        print(f"{k}\ttotal={total}\tblocked={v['blocked']}\tclear={v['clear']}\tstill_undecidable={v['still_undecidable']}")
    tb = sum(v["blocked"] for v in by_kind.values())
    tc = sum(v["clear"] for v in by_kind.values())
    tu = sum(v["still_undecidable"] for v in by_kind.values())
    print(f"TOTAL\tblocked={tb}\tclear={tc}\tstill_undecidable={tu}\tsum={tb+tc+tu}")

    print()
    print("=== by book ===")
    for book in sorted(by_book):
        v = by_book[book]
        total = v["blocked"] + v["clear"] + v["still_undecidable"]
        print(f"{book}\ttotal={total}\tblocked={v['blocked']}\tclear={v['clear']}\tstill_undecidable={v['still_undecidable']}")

    print()
    print("=== fully-resolved books (still_undecidable == 0) -- the T9 onboarding dispatch list ===")
    for book in sorted(by_book):
        v = by_book[book]
        if v["still_undecidable"] == 0:
            print(f"{book}\tblocked={v['blocked']}\tclear={v['clear']}")


if __name__ == "__main__":
    main()
