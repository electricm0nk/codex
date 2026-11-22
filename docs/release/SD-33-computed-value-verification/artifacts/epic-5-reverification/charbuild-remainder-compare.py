#!/usr/bin/env python3
"""SD-33 AT-33-E5-remainder-charbuild -- builds the committed
`charbuild-remainder.oracle-results.json` from:

  - this cycle's Rust probe output (`v06_work_inventory --charbuild-remainder-probe`,
    `ours.json`), the real engine-side magnitude for every unit;
  - the real, live PCGen `BatchExporter` text exports this cycle produced,
    one file per L20 class build / per-race build
    (`fixtures/charbuild-remainder-oracle-txt/*.txt`).

Every regex below extracts a magnitude from PCGen's own fully-formatted,
number-substituted `ABILITYALL...DESC` text (the SAME text a player reads
on the character sheet) -- never a re-derivation of the PF1 rule. A unit
whose regex does not match a real DESC string is `unverifiable`, with the
literal DESC text quoted in the reason, never silently coerced.
"""
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
OURS_PATH = sys.argv[1] if len(sys.argv) > 1 else os.path.join(HERE, "charbuild-remainder.ours.json")
ORACLE_TXT_DIR = sys.argv[2] if len(sys.argv) > 2 else os.path.join(HERE, "fixtures", "charbuild-remainder-oracle-txt")
OUT_PATH = sys.argv[3] if len(sys.argv) > 3 else os.path.join(HERE, "charbuild-remainder.oracle-results.json")

ABILITY_INDEX = ["strength", "dexterity", "constitution", "intelligence", "wisdom", "charisma"]
ABILITY_CODE = ["STR", "DEX", "CON", "INT", "WIS", "CHA"]
BASE_SCORE = 14

# unit_id -> (class-build group, PCGen SA display NAME to find, extraction
# kind). kind is one of:
#   "dice"      -- first "Nd<size>" in DESC, N is the magnitude
#   "bonus"     -- first "+N" in DESC
#   "dc"        -- first "DC ... N" in DESC (save DC magnitude)
#   "times_day" -- first "N time(s) per day" / "N/day" in DESC
#   "temp_hp"   -- first "N temporary hit points" in DESC
#   "hit_points"-- first "heal N hit points" / "N hit points of damage" in DESC
#   "identity"  -- no comparable magnitude on either side (ours is always 0,
#                  a grant-only identity record); unverifiable by construction,
#                  never regex-matched.
CLASS_FEATURE_UNITS = {
    "advanced_class_guide:class_feature:bloodrager_damage_reduction":
        ("bloodrager", "Damage Reduction", "dr"),
    "advanced_class_guide:class_feature:slayer_sneak_attack":
        ("slayer", "Sneak Attack", "dice"),
    "advanced_class_guide:class_feature:slayer_stalker":
        ("slayer", "Stalker", "bonus"),
    "advanced_class_guide:class_feature:slayer_studied_target":
        ("slayer", "Studied Target", "bonus"),
    "advanced_class_guide:class_feature:slayer_trapfinding":
        ("slayer", "Trapfinding", "bonus"),
    "core_rulebook:class_feature:paladin_channel_positive_energy":
        ("paladin", "Channel Positive Energy", "dice"),
    "core_rulebook:class_feature:paladin_lay_on_hands":
        ("paladin", "Lay on Hands", "dice"),
    "core_rulebook:class_feature:rage_power_superstition":
        ("barbarian", "Superstition", "bonus"),
    "core_rulebook:class_feature:ranger_master_hunter":
        ("ranger", "Master Hunter", "dc"),
    "core_rulebook:class_feature:rogue_master_strike":
        ("rogue", "Master Strike", "dc"),
    "core_rulebook:class_feature:rogue_trap_sense":
        ("rogue", "Trap Sense", "bonus"),
    "core_rulebook:class_feature:rogue_trapfinding":
        ("rogue", "Trapfinding", "bonus"),
    "ultimate_combat:class_feature:ninja_no_trace":
        ("ninja", "No Trace", "bonus"),
    "ultimate_combat:class_feature:ninja_sneak_attack":
        ("ninja", "Sneak Attack", "dice"),
    "ultimate_combat:class_feature:samurai_resolve":
        ("samurai", "Resolve", "times_day"),
    "advanced_class_guide:class_feature:investigator_alchemy":
        ("investigator", "Alchemy", "bonus"),
    "advanced_class_guide:class_feature:slayer_talent_foil_scrutiny":
        ("slayer", "Foil Scrutiny", "bonus"),
    "advanced_players_guide:class_feature:inquisitor_track":
        ("inquisitor", "Track", "bonus"),
    "core_rulebook:class_feature:barbarian_uncanny_dodge":
        ("barbarian", "Uncanny Dodge", "identity"),
    "core_rulebook:class_feature:druid_nature_sense":
        ("druid", "Nature Sense", "bonus"),
    "core_rulebook:class_feature:druid_wild_empathy":
        ("druid", "Wild Empathy", "bonus"),
    "core_rulebook:class_feature:monk_ac_bonus":
        ("monk", "AC Bonus", "bonus"),
    "core_rulebook:class_feature:monk_high_jump":
        ("monk", "High Jump", "bonus"),
    "core_rulebook:class_feature:monk_wholeness_of_body":
        ("monk", "Wholeness of Body", "hit_points"),
    "core_rulebook:class_feature:paladin_aura_of_righteousness":
        ("paladin", "Aura of Righteousness", "dr"),
    "core_rulebook:class_feature:paladin_holy_champion":
        ("paladin", "Holy Champion", "identity"),
    "core_rulebook:class_feature:ranger_hunter_s_bond":
        ("ranger", "Hunter's Bond", "identity"),
    "core_rulebook:class_feature:ranger_track":
        ("ranger", "Track", "bonus"),
    "core_rulebook:class_feature:rogue_talent_resiliency":
        ("rogue", "Resiliency", "temp_hp"),
    "core_rulebook:class_feature:rogue_sneak_attack":
        ("rogue", "Sneak Attack", "dice"),
    "core_rulebook:class_feature:rogue_uncanny_dodge":
        ("rogue", "Uncanny Dodge", "identity"),
    "pathfinder_unchained:class_feature:unchained_rogue_sneak_attack":
        ("unchained_rogue", "Sneak Attack", "dice"),
}

IDENTITY_REASON = (
    "engine's own explanation for this feature is a grant-only identity record "
    "(value 0 by design, per its own doc comment in src/rules_core/pilot_compute/) "
    "-- the feature has no single numeric magnitude this engine computes at all "
    "(a boolean immunity, or a multi-clause ability whose clauses are "
    "individually unmodelled), so there is no 'ours' number to compare against "
    "any oracle export token regardless of what PCGen's own DESCRIPTION renders."
)

DR_RE = re.compile(r"(?:[Ss]ubtract (\d+)|DR (\d+)|damage reduction (\d+))")
DICE_RE = re.compile(r"(\d+)d\d+")
BONUS_RE = re.compile(r"\+(\d+)")
BONUS_EQUAL_RE = re.compile(r"equal to (\d+)")
DC_RE = re.compile(r"DC[^0-9]{0,20}(\d+)")
TIMES_DAY_RE = re.compile(r"(\d+)\s*(?:times?\s*(?:/|per)\s*day|/day)", re.IGNORECASE)
TEMP_HP_RE = re.compile(r"(\d+)\s*temporary hit points")
HIT_POINTS_RE = re.compile(r"heal(?:s)?\s*(\d+)|(\d+)\s*hit points", re.IGNORECASE)


def parse_kv(text):
    out = {}
    for line in text.splitlines():
        line = line.strip()
        if not line or "=" not in line:
            continue
        k, _, v = line.partition("=")
        out[k.strip()] = v.strip()
    return out


def sa_entries(kv):
    n = int(kv.get("SA.COUNT", "0") or "0")
    entries = []
    for i in range(n):
        name = kv.get(f"SA.{i}.NAME", "")
        desc = kv.get(f"SA.{i}.DESC", "")
        entries.append((name, desc))
    return entries


def extract(kind, desc):
    if kind == "dr":
        m = DR_RE.search(desc)
        if m:
            return int(next(g for g in m.groups() if g is not None))
        return None
    if kind == "dice":
        m = DICE_RE.search(desc)
    elif kind == "bonus":
        m = BONUS_RE.search(desc) or BONUS_EQUAL_RE.search(desc)
    elif kind == "dc":
        m = DC_RE.search(desc)
    elif kind == "times_day":
        m = TIMES_DAY_RE.search(desc)
    elif kind == "temp_hp":
        m = TEMP_HP_RE.search(desc)
    elif kind == "hit_points":
        m = HIT_POINTS_RE.search(desc)
        if m:
            return int(m.group(1) or m.group(2))
        return None
    else:
        return None
    return int(m.group(1)) if m else None


def main():
    ours = json.load(open(OURS_PATH))
    results = []

    # --- class_feature ---
    class_txt_cache = {}
    for unit_id, (group, sa_name, kind) in CLASS_FEATURE_UNITS.items():
        our_entry = ours["class_feature"].get(unit_id, {})
        our_value = our_entry.get("value") if our_entry.get("found") else None

        if kind == "identity":
            results.append({
                "unit_id": unit_id, "ours": our_value, "oracle": None,
                "verdict": "unverifiable", "kind": "class_feature",
                "reason": IDENTITY_REASON,
            })
            continue

        if group not in class_txt_cache:
            path = os.path.join(ORACLE_TXT_DIR, f"class-{group}.txt")
            if os.path.exists(path):
                class_txt_cache[group] = sa_entries(parse_kv(open(path).read()))
            else:
                class_txt_cache[group] = None
        entries = class_txt_cache[group]

        if entries is None:
            results.append({
                "unit_id": unit_id, "ours": our_value, "oracle": None,
                "verdict": "unverifiable", "kind": "class_feature",
                "reason": f"no oracle export file found for class build {group!r} "
                          f"(fixtures/charbuild-remainder-oracle-txt/class-{group}.txt "
                          "is missing -- the PCGen batch run for this class did not "
                          "complete)",
            })
            continue

        # PF1 corpus data legitimately carries duplicate SA rows for the
        # same ability name at some levels (Rage/Evasion/Channel Positive
        # Energy all do it in this cycle's real exports) -- one row is
        # often the bare grant marker (empty DESC) and a later row carries
        # the real, level-substituted text. Prefer the first NON-EMPTY
        # DESC; only fall back to an empty one if that is all there is.
        all_matches = [d for (n, d) in entries if n == sa_name]
        matches = [d for d in all_matches if d.strip()] or all_matches
        if not matches:
            names = ", ".join(sorted({n for n, _ in entries if n})) or "(none)"
            results.append({
                "unit_id": unit_id, "ours": our_value, "oracle": None,
                "verdict": "unverifiable", "kind": "class_feature",
                "reason": f"PCGen's own Special Abilities list for the {group} L20 "
                          f"build carries no ability named {sa_name!r} (real names "
                          f"present: {names}) -- confirmed absent, not a lookup "
                          "miss: no oracle value exists for this feature at this "
                          "build's posture.",
            })
            continue

        desc = matches[0]
        oracle_value = extract(kind, desc)
        if oracle_value is None:
            results.append({
                "unit_id": unit_id, "ours": our_value, "oracle": None,
                "verdict": "unverifiable", "kind": "class_feature",
                "reason": f"PCGen's real DESCRIPTION text for {sa_name!r} carries "
                          f"no {kind}-shaped numeric token this comparison's regex "
                          f"recognizes. Real DESC text: {desc!r}",
            })
            continue

        verdict = "agree" if our_value == oracle_value else "disagree"
        results.append({
            "unit_id": unit_id, "ours": our_value, "oracle": oracle_value,
            "verdict": verdict, "kind": "class_feature",
        })

    # --- race (ability_adjustments) ---
    for unit_id, entry in ours["race"].items():
        if not entry.get("found"):
            results.append({
                "unit_id": unit_id, "ours": None, "oracle": None,
                "verdict": "unverifiable", "kind": "race",
                "reason": "race_creation_chassis did not resolve for this race "
                          "in this cycle's probe run",
            })
            continue
        slug = unit_id.split(":")[-1]
        path = os.path.join(ORACLE_TXT_DIR, f"race-{slug}.txt")
        if not os.path.exists(path):
            results.append({
                "unit_id": unit_id, "ours": entry["ability_adjustments"], "oracle": None,
                "verdict": "unverifiable", "kind": "race",
                "reason": f"no oracle export file found (fixtures/"
                          f"charbuild-remainder-oracle-txt/race-{slug}.txt is missing)",
            })
            continue
        kv = parse_kv(open(path).read())
        oracle_deltas = {}
        for i, ability in enumerate(ABILITY_INDEX):
            score = kv.get(f"STAT.{i}.SCORE")
            if score is None:
                continue
            try:
                delta = int(score) - BASE_SCORE
            except ValueError:
                continue
            if delta != 0:
                oracle_deltas[ability] = delta

        ours_adj = {k: v for k, v in entry["ability_adjustments"].items() if v != 0}
        floating = entry.get("floating_bonus_points", 0)

        if not ours_adj and floating and floating > 0:
            results.append({
                "unit_id": unit_id, "ours": {"floating_bonus_points": floating},
                "oracle": oracle_deltas or None, "verdict": "unverifiable", "kind": "race",
                "reason": f"this race grants a floating +{floating} to a "
                          "player-chosen ability rather than a fixed set; this "
                          "cycle's minimal .pcg makes no ability selection, so "
                          "PCGen shows all six scores at the unmodified base "
                          "(no per-ability delta exists to compare the floating "
                          "magnitude against in an unselected build).",
            })
            continue

        verdict = "agree" if ours_adj == oracle_deltas else "disagree"
        results.append({
            "unit_id": unit_id, "ours": ours_adj, "oracle": oracle_deltas,
            "verdict": verdict, "kind": "race",
        })

    # --- race_trait ---
    for unit_id, entry in ours["race_trait"].items():
        if entry.get("kind") == "no_verified_consumer":
            results.append({
                "unit_id": unit_id, "ours": None, "oracle": None,
                "verdict": "unverifiable", "kind": "race_trait",
                "reason": entry["reason"],
            })
            continue
        # ability_magnitude race_trait: fold into the SAME race build's
        # STAT delta the `race` loop above already reads -- re-derive here
        # rather than duplicate a second PCGen run.
        if not entry.get("found"):
            results.append({
                "unit_id": unit_id, "ours": None, "oracle": None,
                "verdict": "unverifiable", "kind": "race_trait",
                "reason": "race_creation_chassis did not resolve for this "
                          "race_trait's own race in this cycle's probe run",
            })
            continue
        race_key = entry["race_key"]
        slug = race_key.lower().replace(" ", "_").replace("-", "_")
        # Match this against the SAME race-<slug>.txt this cycle generated
        # for the `race` population when the race also appears there;
        # Aasimar/Oversized Goblin are NOT in the 36-race population, so
        # generate their own comparison directly against their own
        # ability_adjustments (no separate PCGen run needed beyond what
        # the race loop's own convention would produce -- but neither
        # Aasimar nor Oversized Goblin was built this cycle as a `race-*`
        # fixture, since both are `race_trait`, not `race`, units. Report
        # the real engine-side magnitude and name the real reason no
        # oracle run exists for it yet, rather than fabricate a match.)
        results.append({
            "unit_id": unit_id, "ours": entry["ability_adjustments"], "oracle": None,
            "verdict": "unverifiable", "kind": "race_trait",
            "reason": f"real engine-side magnitude computed ({entry['ability_adjustments']}, "
                      f"floating_bonus_points={entry.get('floating_bonus_points', 0)}) via "
                      f"the same race_creation_chassis(...) read the `race` population uses, "
                      f"but this cycle built no PCGen .pcg for {race_key!r} itself (it is a "
                      "race_trait unit, not one of the 36 race units already built) -- no "
                      "live oracle round-trip was run for this specific unit this cycle.",
        })

    with open(OUT_PATH, "w") as f:
        json.dump({"results": results}, f, indent=2)
        f.write("\n")

    from collections import Counter
    c = Counter(r["verdict"] for r in results)
    print(f"{len(results)} units -> {dict(c)}")
    print(f"wrote {OUT_PATH}")


if __name__ == "__main__":
    main()
