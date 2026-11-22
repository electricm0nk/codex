#!/usr/bin/env python3
"""Census of the pool-shaped `class_feature` population (SD-32 T12 §17a).

Re-derives, precisely and reproducibly, the population
`class_feature_pool_catalog.rs` (`REGISTERED_POOL_GROUPS`) is built to serve:
every already-ingested `data/corpus/*/class_feature/**/*.json` record whose
`key` is `" ~ "`-group-qualified (`"<Group> ~ <Member>"`), the same marker
`census_untabled_base_class_feature_roster.py`'s own module doc names as the
pool-exclusion signal.

Mirrors the Rust catalog's own gates exactly (`ENGINE_EFFECT_TOKEN_KEYS`) so
the "magnitude-bearing" count here means precisely "would be refused by
`has_no_engine_effect_token` and therefore needs a real compute function,
not just a rendered description" -- not an independent heuristic.

Per-cycle receipt precedent: prior sizing pass
(`t12-census-widening-followup_cycle-1_cycle_receipt.md` §5) used an
ad-hoc, uncommitted script and reported "~1,913" / "~16,350" / "~6,131" as
approximate. This script is committed and exact so the next cycle does not
re-derive from scratch.

Usage:
    python3 scripts/census_class_feature_pool_population.py [--json]
"""
from __future__ import annotations

import argparse
import glob
import json
import sys
from collections import defaultdict

# Mirrors `class_feature_pool_catalog.rs::ENGINE_EFFECT_TOKEN_KEYS` exactly.
ENGINE_EFFECT_TOKEN_KEYS = {
    "ABILITY", "CSKILL", "SELECT", "AUTO", "SAB", "BONUS", "DEFINE", "ADD", "SPELLS", "DR", "SR",
}

# Groups already modeled by a DEDICATED compute mechanism elsewhere in this
# repo, independent of `class_feature_pool_catalog.rs` -- counting these
# again against the catalog's "unmodeled" figure would double-count.
# `Domain Power` / `Inquisitor Domain`: `decisions.md §23a` (Domain Power
# closes by reading the upstream class link; Inquisitor Domain shares the
# exact same generator per its own module, confirmed by grep below).
ALREADY_MODELED_ELSEWHERE = {
    "Domain Power",
    "Inquisitor Domain",
    "Rogue Talent",  # `class_feature_pool_catalog.rs::REGISTERED_POOL_GROUPS`
    "Rage Power",    # `class_feature_pool_catalog.rs::REGISTERED_POOL_GROUPS`
}

# Groups where only a SUBSET of numeric-magnitude records is already
# modeled elsewhere -- a whole-group exclusion (like
# `ALREADY_MODELED_ELSEWHERE` above) would either under-exclude (miss the
# covered records) or over-exclude (wrongly drop the genuinely-unclosed
# ones), so this is a per-record predicate instead, checked before a
# record is added to the residual.
#
# `Witch Hex` / `Witch Major Hex` / `Witch Grand Hex` (T12 cycle 3 finding,
# following on cycle 2's own discovery-forward): `pilot_compute/mod.rs`'s
# `witch_hex_save_dc` grounds ONE shared DC formula
# (`10 + WitchHexStat + WitchHexAbilityLVL/2`), unconditionally, for every
# Witch regardless of which hex is selected. A hex record's magnitude is
# already covered by that shared formula whenever its ONLY BONUS:VAR
# target is its own per-hex alias of the shared `WitchHexDC` variable
# (`BONUS:VAR|WitchHexDC_<Name>|WitchHexDC`) -- confirmed live against
# every corpus record in these three groups: 54 of 58 numeric-magnitude
# records carry exactly that alias (including Cauldron and Flight, whose
# OWN extra bonuses -- Craft (Alchemy), Swim -- are ALSO separately
# hand-grounded in `ground_or_block_witch_class_features`), and the
# remaining 4 (`Bouda's Eye`, `Enemy Ground`, `Mud Witch`, `No Place Like
# Home`) carry their own distinct DEFINE-based magnitude with no such
# alias and are correctly left in the residual.
WITCH_HEX_FAMILY_GROUPS = {"Witch Hex", "Witch Major Hex", "Witch Grand Hex"}


def witch_hex_alias_target(raw_tokens) -> bool:
    """True if this record's numeric magnitude is entirely the shared
    `WitchHexDC_<Name>` alias `witch_hex_save_dc` already grounds (an
    extra, separately-hand-grounded bonus like Cauldron's or Flight's
    does not disqualify it -- only a magnitude the shared DC formula does
    NOT cover would)."""
    if not isinstance(raw_tokens, list):
        return False
    for t in raw_tokens:
        if not isinstance(t, dict) or t.get("key") != "BONUS":
            continue
        v = t.get("value")
        if isinstance(v, str) and v.startswith("VAR|WitchHexDC_") and v.split("|")[2:3] == ["WitchHexDC"]:
            return True
    return False


def is_real_description(value) -> bool:
    if not isinstance(value, str):
        return False
    trimmed = value.strip()
    if not trimmed:
        return False
    lower = trimmed.lower()
    return lower not in (".clear", ".clearall", "[redacted pi]")


def engine_effect_token_keys_present(raw_tokens) -> set[str]:
    if not isinstance(raw_tokens, list):
        return set()
    return {
        t.get("key")
        for t in raw_tokens
        if isinstance(t, dict) and t.get("key") in ENGINE_EFFECT_TOKEN_KEYS
    }


# Of `ENGINE_EFFECT_TOKEN_KEYS`, the subset that actually produces a NUMERIC
# magnitude a real compute function would need to derive (`BONUS`, `DEFINE`
# variable math, or a `%N`-substituted DESC the numeric-substitution scan
# below independently confirms). `ABILITY`/`CSKILL`/`SELECT`/`AUTO`/`SAB`/
# `ADD`/`SPELLS`/`DR`/`SR` are real mechanics too, but most are boolean
# grants/choices, not a scaled number this catalog's "compute shape"
# classification (flat/linear/level+ability/ability-only) applies to --
# reported as a SEPARATE bucket so "needs a compute function" is not
# conflated with "not plain prose".
NUMERIC_MAGNITUDE_KEYS = {"BONUS", "DEFINE"}


def has_percent_substitution(raw_tokens) -> bool:
    if not isinstance(raw_tokens, list):
        return False
    for t in raw_tokens:
        if isinstance(t, dict) and t.get("key") == "DESC":
            v = t.get("value")
            if isinstance(v, str) and "%1" in v:
                return True
    return False


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--json", action="store_true", help="emit machine-readable JSON")
    args = ap.parse_args()

    files = sorted(glob.glob("data/corpus/*/class_feature/**/*.json", recursive=True))

    groups: dict[str, dict] = defaultdict(
        lambda: {
            "total": 0,
            "any_engine_effect_token": 0,
            "numeric_magnitude": 0,
            "catalog_servable_text_only": 0,
            "books": set(),
        }
    )
    total_qualified = 0
    total_any_effect = 0
    total_numeric = 0
    total_text_only = 0
    malformed = 0
    witch_hex_family_alias_covered = 0

    for f in files:
        try:
            doc = json.load(open(f, encoding="utf-8"))
        except Exception:
            malformed += 1
            continue
        data = doc.get("data", {})
        key = data.get("key")
        if not isinstance(key, str) or " ~ " not in key:
            continue
        group = key.split(" ~ ", 1)[0]
        total_qualified += 1
        book = f.split("/")[1]
        g = groups[group]
        g["total"] += 1
        g["books"].add(book)
        raw_tokens = data.get("raw_tokens")
        effect_keys = engine_effect_token_keys_present(raw_tokens)
        numeric = bool(effect_keys & NUMERIC_MAGNITUDE_KEYS) or has_percent_substitution(raw_tokens)
        if effect_keys:
            g["any_engine_effect_token"] += 1
            total_any_effect += 1
        else:
            g["catalog_servable_text_only"] += 1
            total_text_only += 1
        if numeric:
            g["numeric_magnitude"] += 1
            total_numeric += 1
            if group in WITCH_HEX_FAMILY_GROUPS and witch_hex_alias_target(raw_tokens):
                witch_hex_family_alias_covered += 1

    already_modeled_records = sum(
        g["total"] for name, g in groups.items() if name in ALREADY_MODELED_ELSEWHERE
    )
    already_modeled_numeric = sum(
        g["numeric_magnitude"] for name, g in groups.items() if name in ALREADY_MODELED_ELSEWHERE
    )
    residual_numeric = total_numeric - already_modeled_numeric - witch_hex_family_alias_covered

    result = {
        "files_scanned": len(files),
        "malformed_json": malformed,
        "distinct_group_qualified_names": len(groups),
        "total_group_qualified_records": total_qualified,
        "any_engine_effect_token_records": total_any_effect,
        "catalog_servable_text_only_records": total_text_only,
        "numeric_magnitude_records": total_numeric,
        "already_modeled_elsewhere_groups": sorted(ALREADY_MODELED_ELSEWHERE),
        "already_modeled_elsewhere_records": already_modeled_records,
        "already_modeled_elsewhere_numeric_magnitude": already_modeled_numeric,
        "witch_hex_family_groups": sorted(WITCH_HEX_FAMILY_GROUPS),
        "witch_hex_family_alias_covered_numeric_magnitude": witch_hex_family_alias_covered,
        "residual_numeric_magnitude_needing_compute": residual_numeric,
    }

    if args.json:
        # top groups by numeric-magnitude count, for triage
        top = sorted(groups.items(), key=lambda kv: -kv[1]["numeric_magnitude"])[:60]
        result["top_groups_by_numeric_magnitude"] = [
            {
                "group": name,
                "total": g["total"],
                "numeric_magnitude": g["numeric_magnitude"],
                "any_engine_effect_token": g["any_engine_effect_token"],
                "catalog_servable_text_only": g["catalog_servable_text_only"],
                "books": sorted(g["books"]),
            }
            for name, g in top
        ]
        print(json.dumps(result, indent=2))
    else:
        print("class_feature pool-shaped population census (decisions.md §17/§17a)")
        print(f"  files scanned                                {result['files_scanned']:>7}")
        print(f"  malformed JSON                                {result['malformed_json']:>7}")
        print(f"  distinct ' ~ '-group-qualified names          {result['distinct_group_qualified_names']:>7}")
        print(f"  total group-qualified records                 {result['total_group_qualified_records']:>7}")
        print(f"  catalog-servable text-only (no engine token)  {result['catalog_servable_text_only_records']:>7}")
        print(f"  any engine-effect token (ABILITY/CSKILL/SELECT/AUTO/SAB/BONUS/DEFINE/ADD/SPELLS/DR/SR)")
        print(f"    records                                     {result['any_engine_effect_token_records']:>7}")
        print(f"  numeric magnitude (BONUS/DEFINE var math, or %N-substituted DESC)")
        print(f"    records                                     {result['numeric_magnitude_records']:>7}")
        print(f"  already modeled elsewhere (groups: {', '.join(sorted(ALREADY_MODELED_ELSEWHERE))})")
        print(f"    records                                     {result['already_modeled_elsewhere_records']:>7}")
        print(f"    of which numeric-magnitude                  {result['already_modeled_elsewhere_numeric_magnitude']:>7}")
        print(f"  Witch Hex family alias-covered (witch_hex_save_dc, T12 cycle-3 fix)")
        print(f"    groups: {', '.join(sorted(WITCH_HEX_FAMILY_GROUPS))}")
        print(f"    numeric-magnitude records covered           {result['witch_hex_family_alias_covered_numeric_magnitude']:>7}")
        print(f"  RESIDUAL numeric-magnitude needing compute    {result['residual_numeric_magnitude_needing_compute']:>7}")

    return 0


if __name__ == "__main__":
    sys.exit(main())
