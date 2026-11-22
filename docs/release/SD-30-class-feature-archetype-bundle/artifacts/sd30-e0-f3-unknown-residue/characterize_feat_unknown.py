#!/usr/bin/env python3
"""SD30-E0-F3: characterize feat kind's `unknown` residue using the same
option-pool / genuinely-unreachable / unclustered-remainder method
decisions.md #38 established for class_feature. Reads the live corpus-source
PCGen .lst lines (not the stored work-inventory reason text alone) to derive
the structural bucket for each of the 367 units, per-unit, with the raw line
kept in the artifact for audit.
"""
import json
import collections

WORK_INV = "docs/work-inventory.json"

FILE_PATHS = {
    "up_feats.lst": "/home/ubuntu/workspace/repos/pcgen/data/pathfinder/dreamscarred_press/ultimate_psionics/up_feats.lst",
    "arg_feats.lst": "/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/advanced_race_guide/arg_feats.lst",
    "cr_feats.lst": "/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_feats.lst",
    "um_feats.lst": "/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_magic/um_feats.lst",
    "acg_feats.lst": "/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/advanced_class_guide/acg_feats.lst",
    "uc_feats.lst": "/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_combat/uc_feats.lst",
    "apg_feats.lst": "/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_feats.lst",
    "uw_feats.lst": "/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_wilderness/uw_feats.lst",
    "ui_feats.lst": "/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_intrigue/ui_feats.lst",
    "pu_feats.lst": "/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/pathfinder_unchained/pu_feats.lst",
    "um_feats_wordsofpower.lst": "/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/ultimate_magic/um_feats_wordsofpower.lst",
}

# PROBE_CLASSES from src/bin/v06_work_inventory.rs (line ~127) -- the fixed
# roster the feat-effect probe sweeps. A feat that only manifests on a class
# outside this roster can never show a computed delta under the CURRENT
# probe, regardless of whether its own engine mechanism is wired.
PROBE_CLASSES = {"fighter", "barbarian", "monk", "wizard", "swashbuckler"}

# Resource-pool class ownership, derived from each BONUS:ABILITYPOOL /
# SAB:... token's own pool name where unambiguous from Pathfinder RAW
# (documented per-cluster in the artifact, not asserted blind).
POOL_OWNER_CLASS = {
    "rage power": "barbarian", "arcane pool": "magus", "arcanist exploit": "arcanist",
    "bane": "ranger", "blade skill": "aegis/soulknife (psionic)", "bombs": "alchemist",
    "cantrips or orisons": "wizard/cleric (any 0-level-caster)", "channel": "cleric",
    "customization": "android/construct-kin", "discovery": "alchemist",
    "elemental assault": "kineticist", "evolution": "summoner (eidolon)", "grit": "gunslinger",
    "hex": "witch", "insight": "investigator", "inspiration": "investigator",
    "investigator talent": "investigator", "lay on hands": "paladin",
    "martial flexibility": "brawler", "mercy": "paladin", "performance": "bard",
    "power known": "psion/psychic (psionic)", "reconfiguration": "android/construct",
    "reservoir": "kineticist/occultist", "revelation": "oracle", "rogue talent": "rogue",
    "slayer talent": "slayer", "stamina": "any (pathfinder unchained variant rule)",
    "strategy": "cavalier/tactician", "summons": "summoner", "terrors": "dread (psionic)",
    "transfer": "occultist/mesmer", "unchained rogue talent": "rogue (unchained)",
    "word": "cleric (words of power variant)", "arcana": "wizard (arcane discovery-adjacent)",
}


def load_units():
    d = json.load(open(WORK_INV))
    return [u for u in d["units"] if u["status"] == "unknown" and u["kind"] == "feat"], d.get("generated_at")


def load_lines():
    cache = {}
    for fname, path in FILE_PATHS.items():
        with open(path, "r", errors="replace") as fh:
            cache[fname] = fh.readlines()
    return cache


import re as _re

# Structural signal detection is token-based only (PCGen .lst fields), never
# a name-word guess -- a name-word heuristic ("contains 'critical'") was
# tried first and produces unverifiable, low-confidence buckets; dropped in
# favour of signals the .lst line itself asserts.
_PREABILITY_POSITIVE_RE = _re.compile(r"(?<!!)PREABILITY:")


def classify(unit, raw_line):
    """Return (top_bucket, shape, subreason) using ONLY decisions.md #38's
    three top-level buckets (option-pool / genuinely-unreachable /
    unclustered-remainder), each with a feat-probe-specific `shape` naming
    which structural signal produced the call."""
    name_lower = unit["name"].lower()
    tokens = raw_line
    corpus_key = unit.get("corpus_key") or ""

    # A PCGen KEY containing " ~ " is PCGen's own explicit named-sub-choice
    # marker (`KEY:Angelic Flesh ~ Brazen`, `KEY:Heavenly Radiance ~
    # Sunbeam`) -- the record IS one option inside a parent class-native
    # chooser feat, the precise shape decisions.md #38 named as its
    # dominant, non-ingest-gap bucket. Checked before PREABILITY: these
    # option rows often ALSO carry a PREABILITY pointing at the parent
    # feat, which would otherwise misroute them into the fixture-capability
    # bucket even though the true shape is option-pool.
    if " ~ " in corpus_key:
        return (
            "option-pool (mechanism real, specific pool-slot ungrounded)",
            "named-sub-choice-key",
            f"corpus_key '{corpus_key}' carries PCGen's explicit chooser-option "
            f"KEY marker (' ~ '); this is one option inside a parent class-native "
            f"chooser feat, decisions.md #38's dominant shape",
        )

    has_abilitypool = "BONUS:ABILITYPOOL" in tokens
    # Positive PREABILITY only: "requires you already have X" is the
    # chooser-pre-selection-gap shape. "!PREABILITY" (requires you do NOT
    # have X) is trivially satisfied by the probe's synthetic characters and
    # is a different shape entirely -- conflating the two was caught live
    # re-checking `Amateur Investigator` (`!PREABILITY:1,...Inspiration`,
    # negated) against this classifier's first pass, which had wrongly
    # bucketed it as chooser-pre-selection-unreachable. retro.py near-miss
    # event emitted for this catch (see receipt).
    has_preability = bool(_PREABILITY_POSITIVE_RE.search(tokens))
    has_choose = "CHOOSE:" in tokens and "CHOOSE:NOCHOICE" not in tokens
    # PRESTAT/PRESKILL name an ability-score or skill-rank floor the probe's
    # fixed fixture character sheet was not built to satisfy per-feat (the
    # fixture has ONE stat block per PROBE_CLASSES entry, not a
    # per-candidate-feat-tailored one) -- a prerequisite-shaped gap, same
    # standing as PREABILITY: the fixture, not the corpus, is what's narrow.
    has_prereq_stat_or_skill = bool(_re.search(r"PRESTAT:|PRESKILL:", tokens))

    # Bucket A: option-pool analogue -- mirrors decisions.md #38's dominant
    # shape (mechanism real and wired; a *specific* option/pool-slot's own
    # magnitude is what's uncomputed, not the pool itself).
    if has_abilitypool or name_lower.startswith("extra "):
        pool_key = None
        for k in POOL_OWNER_CLASS:
            if k in name_lower.replace("extra ", ""):
                pool_key = k
                break
        owner = POOL_OWNER_CLASS.get(pool_key, "unresolved-owner")
        probe_covers = any(c in owner for c in PROBE_CLASSES)
        return (
            "option-pool (mechanism real, specific pool-slot ungrounded)",
            "resource-pool-expansion",
            f"BONUS:ABILITYPOOL/resource-pool grant; owning class '{owner}' "
            f"{'IS' if probe_covers else 'is NOT'} in PROBE_CLASSES "
            f"{sorted(PROBE_CLASSES)}",
        )
    if has_choose:
        return (
            "option-pool (mechanism real, specific pool-slot ungrounded)",
            "inline-choose",
            "CHOOSE: token present on the feat record itself; probe's fixed "
            "PROBE_SELECTIONS roster is generic and does not exercise this "
            "feat's own named choice list",
        )

    # Bucket B: genuinely-unreachable by the CURRENT probe fixture -- needs
    # new probe/engine capability (a richer fixture), not corpus ingest.
    if has_preability:
        return (
            "genuinely-unreachable (needs new probe-fixture capability)",
            "chooser-pre-selection-gap",
            "PREABILITY names a prior chooser pick the fixed probe fixture never makes",
        )
    if has_prereq_stat_or_skill:
        return (
            "genuinely-unreachable (needs new probe-fixture capability)",
            "prereq-stat-or-skill-gap",
            "PRESTAT/PRESKILL floor the probe's fixed per-class stat block was not built "
            "to satisfy on a per-feat basis",
        )

    # Bucket C: unclustered remainder -- no structural signal matched; open,
    # not silently dropped, same standing as decisions.md #38's own
    # unclustered residue (908 KEY: prefixes at that snapshot).
    return (
        "unclustered-remainder",
        "no-structural-signal",
        "no ABILITYPOOL/CHOOSE/PREABILITY/PRESTAT/PRESKILL token matched this feat's .lst line",
    )


def find_raw_line(fname, source_line, corpus_key, lines_cache):
    """corpus_key is the record's PCGen KEY when the record declares an
    explicit `KEY:` token (sub-choice option rows under a parent chooser,
    e.g. `KEY:Angelic Flesh ~ Brazen`), otherwise it equals the leading
    Ability-Name field. Try both, direct-index first, then whole-file scan."""
    lines = lines_cache[fname]
    idx = source_line - 1
    key_token = f"KEY:{corpus_key}"

    def matches(ln):
        first_field = ln.split("\t")[0].strip()
        return first_field == corpus_key or key_token in ln

    if 0 <= idx < len(lines) and matches(lines[idx]):
        return lines[idx].rstrip("\n")
    for ln in lines:
        if matches(ln):
            return ln.rstrip("\n")
    return None


def main():
    units, generated_at = load_units()
    lines_cache = load_lines()

    bucket_counts = collections.Counter()
    shape_counts = collections.Counter()
    per_unit = []
    unresolved_line = 0

    for u in units:
        raw = find_raw_line(u["source_file"], u["source_line"], u["corpus_key"], lines_cache)
        if raw is None:
            unresolved_line += 1
            bucket, shape, sub = (
                "unclustered-remainder",
                "source-line-not-relocated",
                "source .lst line could not be re-located by corpus_key at/near source_line",
            )
        else:
            bucket, shape, sub = classify(u, raw)
        bucket_counts[bucket] += 1
        shape_counts[(bucket, shape)] += 1
        per_unit.append({
            "id": u["id"],
            "name": u["name"],
            "book": u["book"],
            "source_file": u["source_file"],
            "source_line": u["source_line"],
            "bucket": bucket,
            "shape": shape,
            "sub_reason": sub,
        })

    print(f"work-inventory generated_at: {generated_at}")
    print(f"total feat unknown units: {len(units)}")
    print(f"units whose source .lst line could not be re-located: {unresolved_line}")
    print()
    print("top bucket counts (decisions.md #38 taxonomy):")
    for k, v in bucket_counts.most_common():
        print(f"  {v:4d}  {k}")
    print()
    print("shape sub-counts:")
    for (b, s), v in sorted(shape_counts.items(), key=lambda x: -x[1]):
        print(f"  {v:4d}  {b} :: {s}")
    print()
    print("=== sample per shape (2 each) ===")
    seen = collections.defaultdict(list)
    for pu in per_unit:
        key = (pu["bucket"], pu["shape"])
        if len(seen[key]) < 2:
            seen[key].append(pu)
    for (b, s), items in seen.items():
        print(f"-- {b} :: {s} --")
        for it in items:
            print(f"   {it['id']}: {it['sub_reason']}")

    out_path = "/tmp/claude-1000/-home-ubuntu-workspace-repos-codex/d9c38510-724f-408f-b3c9-273134333e9d/scratchpad/feat_unknown_characterization.json"
    with open(out_path, "w") as f:
        json.dump({
            "generated_at": generated_at,
            "total": len(units),
            "unresolved_source_line": unresolved_line,
            "bucket_counts": dict(bucket_counts),
            "shape_counts": {f"{b} :: {s}": v for (b, s), v in shape_counts.items()},
            "units": per_unit,
        }, f, indent=2)


if __name__ == "__main__":
    main()
